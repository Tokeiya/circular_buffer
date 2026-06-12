use crate::resizable::ResizableIndexCoordinator;
use crate::CircularBuffer;
use crate::Iter;
use crate::IterMut;
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};
#[cfg(test)]
use std::{cell::Cell, rc::Rc};

pub struct Buffer<T, C: ResizableIndexCoordinator> {
	#[cfg(test)]
	probe: Option<Rc<Cell<usize>>>,
	pub(super) storage: Vec<MaybeUninit<T>>,
	pub(super) coordinator: C,
}

impl<T, C: ResizableIndexCoordinator> Buffer<T, C> {
	pub fn new(coordinator: C) -> Self {
		let mut vec = Vec::with_capacity(coordinator.capacity());
		for _ in 0..coordinator.capacity() {
			vec.push(MaybeUninit::uninit());
		}
		Self {
			#[cfg(test)]
			probe: None,
			storage: vec,
			coordinator,
		}
	}
}

impl<T, C: ResizableIndexCoordinator> Index<usize> for Buffer<T, C> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		match self.coordinator.virtual_to_real(index) {
			Ok(i) => unsafe { self.storage.get_unchecked(i).assume_init_ref() },
			Err(_) => panic!("Index out of range"),
		}
	}
}

impl<T, C: ResizableIndexCoordinator> IndexMut<usize> for Buffer<T, C> {
	fn index_mut(&mut self, index: usize) -> &mut T {
		match self.coordinator.virtual_to_real(index) {
			Ok(i) => unsafe { self.storage.get_unchecked_mut(i).assume_init_mut() },
			Err(_) => panic!("Index out of range"),
		}
	}
}

impl<T, C: ResizableIndexCoordinator> CircularBuffer<T> for Buffer<T, C> {
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
		self.coordinator.capacity()
	}

	//noinspection DuplicatedCode
	fn enqueue(&mut self, item: T) {
		if self.len() < self.capacity() {
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

	//noinspection DuplicatedCode
	fn dequeue(&mut self) -> Option<T> {
		if self.coordinator.len() == 0 {
			None
		} else {
			let index = self.coordinator.virtual_to_real(0).unwrap();
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
		IterMut::new(self, ptr, self.coordinator.clone())
	}

	fn len(&self) -> usize {
		self.coordinator.len()
	}
}

impl<T, C: ResizableIndexCoordinator> Drop for Buffer<T, C> {
	//noinspection DuplicatedCode
	fn drop(&mut self) {
		for i in 0..self.coordinator.len() {
			#[cfg(test)]
			{
				if let Some(p) = &self.probe {
					let i = p.get() + 1;
					p.set(i)
				}
			}
			unsafe {
				self.storage[self.coordinator.virtual_to_real(i).unwrap()].assume_init_drop();
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::index_coordinator::IndexCoordinator;
	use crate::resizable::index_coordinator_tests::IndexCoordinatorTestExtension;
	use crate::resizable::Pow2IndexCoordinator;
	use crate::test_shared::{Monitor, MonitorGenerator, Probe};
	use std::assert_matches;
	use std::mem::MaybeUninit;
	use std::panic::{catch_unwind, set_hook, take_hook, AssertUnwindSafe};
	use std::sync::{LazyLock, Mutex};
	
	type ProbeFixture = Buffer<Probe, Pow2IndexCoordinator>;
	type UsizeFixture = Buffer<usize, Pow2IndexCoordinator>;
	type DropCounter = Rc<Cell<usize>>;
	const CAPACITY: usize = 8;

	fn probe_fixture() -> (ProbeFixture, DropCounter) {
		let coordinator = Pow2IndexCoordinator::try_new(CAPACITY).unwrap();

		let probe = Rc::new(Cell::new(0usize));
		(
			Buffer {
				probe: probe.clone().into(),
				storage: (0..CAPACITY).map(|_| MaybeUninit::<_>::uninit()).collect(),
				coordinator,
			},
			probe.clone(),
		)
	}

	fn usize_fixture() -> UsizeFixture {
		let coordinator = Pow2IndexCoordinator::try_new(CAPACITY).unwrap();
		Buffer {
			probe: Default::default(),
			storage: vec![const { MaybeUninit::uninit() }; CAPACITY],
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
		let mut fixture = ProbeFixture::new(Pow2IndexCoordinator::try_new(CAPACITY).unwrap());
		assert_eq!(fixture.coordinator.ref_capacity(), &CAPACITY);
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

			assert_eq!(target.capacity(), CAPACITY);
			assert_eq!(target.coordinator.ref_capacity(), &CAPACITY)
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
			fixture.storage[c.virtual_to_real(i).unwrap()] = MaybeUninit::new(i);
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

			assert_eq!(target.capacity(), CAPACITY);
			assert_eq!(target.coordinator.ref_capacity(), &CAPACITY)
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
}
