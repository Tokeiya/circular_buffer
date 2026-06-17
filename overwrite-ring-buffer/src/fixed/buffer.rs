#[cfg(test)]
use std::{cell::Cell, rc::Rc};

use super::FixedIndexCoordinator;
use crate::Iter;
use crate::IterMut;
use crate::circular_buffer::CircularBuffer;
use crate::drop_gurd::DropGuard;
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

pub struct Buffer<T, C: FixedIndexCoordinator<N>, const N: usize> {
	#[cfg(test)]
	probe: Option<Rc<Cell<usize>>>,
	pub(super) storage: [MaybeUninit<T>; N],
	pub(super) coordinator: C,
}

impl<T, C: FixedIndexCoordinator<N>, const N: usize> Default for Buffer<T, C, N> {
	fn default() -> Self {
		Self {
			storage: [const { MaybeUninit::uninit() }; N],
			coordinator: C::default(),
			#[cfg(test)]
			probe: None,
		}
	}
}

impl<T, C: FixedIndexCoordinator<N>, const N: usize> Index<usize> for Buffer<T, C, N> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		match self.coordinator.resolve_index(index) {
			Ok(i) => unsafe { self.storage[i].assume_init_ref() },
			Err(_) => panic!("Index out of bounds"),
		}
	}
}

impl<T, C: FixedIndexCoordinator<N>, const N: usize> IndexMut<usize> for Buffer<T, C, N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match self.coordinator.resolve_index(index) {
			Ok(i) => unsafe { self.storage[i].assume_init_mut() },
			Err(_) => panic!("Index out of bounds"),
		}
	}
}

impl<T, C: FixedIndexCoordinator<N>, const N: usize> CircularBuffer<T> for Buffer<T, C, N> {
	type Iter<'a>
		= Iter<'a, T, C>
	where
		T: 'a,
		Self: 'a;

	type MutIter<'a>
		= IterMut<'a, T, C>
	where
		T: 'a,
		Self: 'a;

	fn capacity(&self) -> usize {
		N
	}

	fn enqueue(&mut self, item: T) {
		if self.len() < N {
			self.coordinator.enqueue_index();
			self.storage[self.coordinator.tail_index().unwrap()].write(item);
		} else {
			let index = self.coordinator.head_index().unwrap();

			let recent = unsafe { self.storage[index].assume_init_read() };
			self.storage[index].write(item);
			self.coordinator.enqueue_index();

			drop(recent);
		}
	}

	fn dequeue(&mut self) -> Option<T> {
		if self.coordinator.len() == 0 {
			None
		} else {
			let index = self.coordinator.resolve_index(0).unwrap();
			let ret = unsafe {
				std::mem::replace(&mut self.storage[index], MaybeUninit::uninit()).assume_init()
			};

			self.coordinator.dequeue_index().unwrap();

			Some(ret)
		}
	}

	fn iter(&self) -> Self::Iter<'_> {
		Iter::new(&self.storage, self.coordinator.clone())
	}

	fn iter_mut(&mut self) -> Self::MutIter<'_> {
		let ptr = self.storage.as_mut_ptr();
		Self::MutIter::new(self, ptr, self.coordinator.clone())
	}

	fn len(&self) -> usize {
		self.coordinator.len()
	}

	fn clear(&mut self) {
		let recent_coordinator = std::mem::replace(&mut self.coordinator, C::default());
		let mut guard = DropGuard::new(&mut self.storage, recent_coordinator);

		while guard.drop_next() {}
	}
}

impl<T, C: FixedIndexCoordinator<N>, const N: usize> Buffer<T, C, N> {
	#[allow(dead_code)]
	#[cfg(test)]
	fn new_with_probe(probe: Rc<Cell<usize>>) -> Self {
		let mut s = Self::default();
		s.probe = Some(probe);
		s
	}
}

impl<T, C: FixedIndexCoordinator<N>, const N: usize> Drop for Buffer<T, C, N> {
	//noinspection DuplicatedCode
	fn drop(&mut self) {
		let mut guard = DropGuard::new(&mut self.storage, self.coordinator.clone());

		while guard.drop_next() {
			#[cfg(test)]
			{
				if let Some(p) = &self.probe {
					let i = p.get() + 1;
					p.set(i)
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::circular_buffer::CircularBuffer;
	use crate::fixed::index_coordinator_test::IndexCoordinatorTestExtensions;
	use crate::fixed::{Buffer, Pow2IndexCoordinator};
	use crate::index_coordinator::IndexCoordinator;
	use crate::test_shared::{Monitor, MonitorGenerator, Probe};
	use std::assert_matches;
	use std::cell::Cell;
	use std::mem::MaybeUninit;
	use std::ops::{Index, IndexMut};
	use std::panic::{AssertUnwindSafe, catch_unwind, set_hook, take_hook};
	use std::ptr::drop_in_place;
	use std::rc::Rc;
	use std::sync::{LazyLock, Mutex};

	type ProbeFixture = Buffer<Probe, Pow2IndexCoordinator<CAPACITY>, CAPACITY>;
	type UsizeFixture = Buffer<usize, Pow2IndexCoordinator<CAPACITY>, CAPACITY>;
	type DropCounter = Rc<Cell<usize>>;
	const CAPACITY: usize = 8;

	fn probe_fixture() -> (ProbeFixture, DropCounter) {
		let coordinator = Pow2IndexCoordinator::<CAPACITY>::default();

		let probe = Rc::new(Cell::new(0usize));
		(
			Buffer {
				probe: probe.clone().into(),
				storage: [const { MaybeUninit::uninit() }; CAPACITY],
				coordinator,
			},
			probe.clone(),
		)
	}

	fn usize_fixture() -> UsizeFixture {
		let coordinator = Pow2IndexCoordinator::<CAPACITY>::default();

		Buffer {
			probe: Default::default(),
			storage: [MaybeUninit::uninit(); CAPACITY],
			coordinator,
		}
	}

	fn create_monitor(size: usize, generator: Option<&mut MonitorGenerator>) -> Vec<Monitor> {
		match generator {
			None => {
				let mut factory = MonitorGenerator::default();
				(0..size).map(|_| factory.generate()).collect()
			}
			Some(g) => (0..size).map(|_| g.generate()).collect(),
		}
	}

	static BLOCKER: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
	fn should_panic(f: impl FnOnce()) {
		let _token = BLOCKER.lock().unwrap();
		let recent = take_hook();
		set_hook(Box::new(move |_info| {}));

		let result = catch_unwind(AssertUnwindSafe(f));
		assert!(result.is_err(), "Expected panic, but none occurred");

		set_hook(recent);
	}

	#[test]
	fn new() {
		let mut fixture = ProbeFixture::default();
		assert_eq!(fixture.coordinator.mut_head(), &mut 0usize);
		assert_eq!(fixture.coordinator.mut_len(), &mut 0usize);
		assert_eq!(fixture.storage.len(), CAPACITY);
	}

	#[test]
	fn index_enqueue() {
		fn check(target: &UsizeFixture, n: usize, offset: usize) {
			for idx in 0..n {
				assert_eq!(*UsizeFixture::index(target, idx), idx + offset);
			}

			should_panic(|| _ = UsizeFixture::index(target, n));
		}

		let mut fixture = usize_fixture();
		should_panic(|| _ = fixture[0]);

		for i in 1..=CAPACITY {
			fixture.enqueue(i);
			check(&fixture, i, 1);
		}

		for i in 1..=CAPACITY {
			fixture.enqueue(i + CAPACITY);
			check(&fixture, CAPACITY, i + 1);
		}

		*fixture.coordinator.mut_head() = CAPACITY / 2;
		let c = fixture.coordinator.clone();

		for i in 0..CAPACITY {
			fixture.storage[c.resolve_index(i).unwrap()] = MaybeUninit::new(i);
		}

		for i in 0..CAPACITY {
			assert_eq!(*UsizeFixture::index(&fixture, i), i);
		}
	}

	#[test]
	fn index_mut_read_write() {
		let mut fixture = usize_fixture();
		should_panic(|| _ = fixture[0]);

		fn check(target: &mut UsizeFixture, n: usize, offset: usize) {
			for idx in 0..n {
				assert_eq!(*UsizeFixture::index_mut(target, idx), idx + offset);
			}

			should_panic(|| _ = UsizeFixture::index_mut(target, n));
		}

		for i in 1..=CAPACITY {
			fixture.enqueue(i);
			check(&mut fixture, i, 1);
		}

		for i in 1..=CAPACITY {
			fixture.enqueue(i + CAPACITY);
			check(&mut fixture, CAPACITY, i + 1);
		}

		*fixture.coordinator.mut_head() = CAPACITY / 2;

		for i in 0..CAPACITY {
			*fixture.index_mut(i) = i;
		}

		for i in 0..CAPACITY {
			assert_eq!(*UsizeFixture::index_mut(&mut fixture, i), i);
		}
	}

	//noinspection DuplicatedCode
	#[test]
	fn index_mut_drop_test() {
		let (mut fixture, _) = probe_fixture();
		let mut generator = MonitorGenerator::default();
		let init = create_monitor(CAPACITY, Some(&mut generator));

		for p in init.iter().map(|m| m.payout_probe()) {
			fixture.enqueue(p);
		}

		let overwrite = create_monitor(CAPACITY, Some(&mut generator));

		for (i, p) in overwrite.iter().map(|m| m.payout_probe()).enumerate() {
			*fixture.index_mut(i) = p;
		}

		assert!(init.iter().all(|x| x.is_dropped()));
		assert!(overwrite.iter().all(|x| !x.is_dropped()));
	}

	#[test]
	fn capacity() {
		let fixture = usize_fixture();
		assert_eq!(fixture.capacity(), CAPACITY);
	}

	//noinspection DuplicatedCode
	#[test]
	fn dequeue() {
		let mut generator = MonitorGenerator::default();
		let (mut fixture, _) = probe_fixture();

		assert_matches!(fixture.dequeue(), None);

		let init = create_monitor(CAPACITY, Some(&mut generator));

		for p in init.iter().map(|m| m.payout_probe()) {
			fixture.enqueue(p);
		}

		for i in 0..CAPACITY {
			assert_eq!(*fixture.coordinator.mut_len(), CAPACITY - i);
			let act = fixture.dequeue().unwrap();
			assert_eq!(act.id(), i);
		}

		assert_eq!(*fixture.coordinator.mut_len(), 0);
		assert!(init.iter().all(|x| x.is_dropped()));
		assert_matches!(fixture.dequeue(), None);
	}

	#[test]
	fn iter() {
		let mut fixture = usize_fixture();
		for i in 0..CAPACITY {
			fixture.enqueue(i);
		}

		for (idx, i) in fixture.iter().enumerate() {
			assert_eq!(*i, idx);
		}
	}

	#[test]
	fn iter_mut() {
		let mut fixture = usize_fixture();
		for i in 0..CAPACITY {
			fixture.enqueue(i);
		}

		for iter in fixture.iter_mut() {
			*iter += 10;
		}

		for (idx, i) in fixture.iter().enumerate() {
			assert_eq!(*i, idx + 10);
		}
	}

	#[test]
	fn len() {
		let mut fixture = usize_fixture();

		for i in 0..CAPACITY {
			assert_eq!(fixture.len(), i);
			fixture.enqueue(i);
		}

		for i in 0..CAPACITY {
			assert_eq!(fixture.len(), CAPACITY);
			fixture.enqueue(i)
		}
	}

	#[test]
	fn drop() {
		let (mut fixture, cnt) = probe_fixture();
		let init = create_monitor(CAPACITY, None);

		for p in init.iter().map(|m| m.payout_probe()) {
			fixture.enqueue(p);
		}

		std::mem::drop(fixture);

		assert!(init.iter().all(|x| x.is_dropped()));
		assert_eq!(cnt.take(), CAPACITY);
	}

	//noinspection DuplicatedCode
	#[test]
	fn drop_panic() {
		let mut generator = MonitorGenerator::default();
		let monitor: [Monitor; CAPACITY] = std::array::from_fn(|_| generator.generate());
		let mut fixture = ProbeFixture::default();

		for elem in monitor.iter() {
			if elem.id() == 5 {
				fixture.enqueue(elem.payout_probe_with_behaviour(|item| {
					panic!("Scheduled panic on drop for item with id {}", item.id())
				}));
			} else {
				fixture.enqueue(elem.payout_probe());
			}
		}

		let result = catch_unwind(AssertUnwindSafe(|| std::mem::drop(fixture)));
		assert!(result.is_err(), "Expected panic, but none occurred");

		for elem in monitor.iter() {
			if elem.id() == 5 {
				assert!(
					!elem.is_dropped(),
					"Item with id 5 should not be dropped due to panic"
				);
			} else {
				assert!(
					elem.is_dropped(),
					"Item with id {} should be dropped",
					elem.id()
				);
			}
		}
	}

	#[test]
	fn clear() {
		let (mut fixture, _) = probe_fixture();
		let init = create_monitor(CAPACITY, None);

		for p in init.iter().map(|m| m.payout_probe()) {
			fixture.enqueue(p);
		}

		fixture.clear();

		assert!(init.iter().all(|x| x.is_dropped()));
	}

	#[test]
	fn clear_panic() {
		let mut generator = MonitorGenerator::default();
		let monitor: [Monitor; CAPACITY] = std::array::from_fn(|_| generator.generate());
		let mut fixture = ProbeFixture::default();

		for elem in monitor.iter() {
			if elem.id() == 5 {
				fixture.enqueue(elem.payout_probe_with_behaviour(|item| {
					panic!("Scheduled panic on drop for item with id {}", item.id())
				}));
			} else {
				fixture.enqueue(elem.payout_probe());
			}
		}

		let result = catch_unwind(AssertUnwindSafe(|| fixture.clear()));
		assert!(result.is_err(), "Expected panic, but none occurred");

		for elem in monitor.iter() {
			if elem.id() == 5 {
				assert!(
					!elem.is_dropped(),
					"Item with id 5 should not be dropped due to panic"
				);
			} else {
				assert!(
					elem.is_dropped(),
					"Item with id {} should be dropped",
					elem.id()
				);
			}
		}
	}
}
