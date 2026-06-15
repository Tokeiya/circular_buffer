#[cfg(test)]
use std::{cell::Cell, rc::Rc};

use super::FixedIndexCoordinator;
use crate::circular_buffer::CircularBuffer;
use crate::drop_gurd::DropGuard;
use crate::Iter;
use crate::IterMut;
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
		todo!()
	}
}

impl<T, C: FixedIndexCoordinator<N>, const N: usize> Buffer<T, C, N> {
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
	use super::*;
	use crate::fixed::Pow2IndexCoordinator;
	use crate::index_coordinator::IndexCoordinator;
	use crate::test_shared::{Monitor, MonitorGenerator, Probe};
	use std::cell::Cell;
	use std::panic::{catch_unwind, AssertUnwindSafe};
	use std::rc::Rc;
	
	const SIZE: usize = 8;
	type Fixture = Buffer<u8, Pow2IndexCoordinator<SIZE>, SIZE>;

	#[test]
	fn default() {
		let fixture = Fixture::default();
		// assert_eq!(fixture.storage.len(), 0);
		assert_eq!(fixture.coordinator.len(), 0);
	}

	#[test]
	fn capacity() {
		let fixture = Fixture::default();
		assert_eq!(fixture.capacity(), 8);
	}

	#[test]
	fn push_index_len() {
		let mut fixture = Fixture::default();
		assert!(catch_unwind(AssertUnwindSafe(|| _ = fixture[0])).is_err());

		for i in 0..SIZE {
			fixture.enqueue(i as u8);
			assert_eq!(fixture.len(), i + 1);
		}

		for i in 0..SIZE {
			assert_eq!(fixture[i], i as u8);
		}

		assert!(catch_unwind(AssertUnwindSafe(|| _ = fixture[SIZE])).is_err());

		const OFFSET: u8 = 100;

		for i in 0..SIZE {
			fixture.enqueue(i as u8 + OFFSET);
			assert_eq!(fixture[7], i as u8 + OFFSET);
			if i == 7 {
				assert_eq!(fixture[0], 100);
			} else {
				assert_eq!(fixture[0], (i as u8) + 1);
			}
		}

		for i in 0..SIZE {
			assert_eq!(fixture[i], (i as u8) + OFFSET);
			assert_eq!(fixture.len(), SIZE);
		}

		fixture.enqueue(42);
		assert_eq!(fixture[7], 42);
		assert_eq!(fixture[0], 101);
	}

	#[test]
	fn index_mut() {
		let mut fixture = Fixture::default();
		assert!(catch_unwind(AssertUnwindSafe(|| _ = fixture[0])).is_err());

		for i in 0..SIZE {
			fixture.enqueue(i as u8);
			assert_eq!(fixture[i], i as u8);
		}

		for i in 0..SIZE {
			fixture[i] = i as u8 + 10;
			assert_eq!(fixture[i], i as u8 + 10);
		}
	}

	#[test]
	#[should_panic]
	fn index_mut_out_of_range() {
		let mut fixture = Fixture::default();
		fixture[SIZE] = 42;
	}

	#[test]
	fn iter() {
		let mut fixture = Fixture::default();

		for i in 0..SIZE {
			{
				assert_eq!(fixture.iter().len(), i);
				let mut iter = fixture.iter();
				for exp in 0..i {
					assert_eq!(*iter.next().unwrap(), exp as u8);
				}
			}

			fixture.enqueue(i as u8);
		}

		assert_eq!(fixture.iter().len(), SIZE);

		for (e, i) in fixture.iter().enumerate() {
			assert_eq!(*i, e as u8);
		}
	}

	#[test]
	fn dequeue() {
		let mut fixture = Fixture::default();
		assert!(fixture.dequeue().is_none());

		for i in 0..SIZE {
			fixture.enqueue(i as u8);
		}

		for i in 0..SIZE {
			let act = fixture.dequeue().unwrap();
			assert_eq!(act, i as u8);
		}

		assert_eq!(fixture.dequeue(), None);
	}

	#[test]
	fn complex_enqueue() {
		let mut fixture = Fixture::default();
		for i in 0..100 {
			fixture.enqueue(i as u8);
		}

		for i in 0..SIZE {
			println!("[{i}]={}", fixture[i]);
		}
		println!("----");

		println!("dequeue:{}", fixture.dequeue().unwrap());

		fixture.enqueue(8);

		print!("vec![");
		for i in 0..SIZE {
			print!("{}, ", fixture[i]);
		}
		println!("]");

		for (idx, i) in [93, 94, 95, 96, 97, 98, 99, 8].iter().enumerate() {
			assert_eq!(fixture[idx], *i);
		}
	}

	#[test]
	fn complex() {
		let mut fixture = Fixture::default();
		for i in 0..100 {
			fixture.enqueue(i as u8);
		}

		for i in 92..100 {
			assert_eq!(fixture.dequeue().unwrap(), i as u8);
		}

		for i in 0..100 {
			fixture.enqueue(i as u8);
		}

		for (exp, act) in fixture.iter().enumerate() {
			assert_eq!(*act, exp as u8 + 92);
		}
	}

	#[test]
	fn drop() {
		for n in 0..SIZE {
			let probe = Rc::new(Cell::new(0usize));
			let mut fixture = Fixture::new_with_probe(probe.clone());

			for i in 0..n {
				fixture.enqueue(i as u8);
			}

			std::mem::drop(fixture);
			assert_eq!(probe.get(), n);
		}
	}

	#[test]
	fn iter_mut() {
		let mut fixture = Fixture::default();
		for i in 0..SIZE {
			fixture.enqueue(i as u8);
		}

		for iter in fixture.iter_mut() {
			*iter += 10;
		}

		for (idx, i) in fixture.iter().enumerate() {
			assert_eq!(*i, idx as u8 + 10);
		}
	}

	#[test]
	fn drop_check() {
		let mut factory = MonitorGenerator::default();
		let monitor: [Monitor; SIZE] = std::array::from_fn(|_| factory.generate());

		let mut fixture = Buffer::<Probe, Pow2IndexCoordinator<SIZE>, SIZE>::default();
		for m in monitor.as_slice().iter() {
			fixture.enqueue(m.payout_probe())
		}
		std::mem::drop(fixture);

		assert!(monitor.iter().all(|m| m.is_dropped()))
	}

	#[test]
	fn index_mut_overwrite() {
		const NUM: usize = 12;

		let mut factory = MonitorGenerator::default();
		let monitor: [Monitor; NUM] = std::array::from_fn(|_| factory.generate());

		let mut fixture = Buffer::<Probe, Pow2IndexCoordinator<SIZE>, SIZE>::default();
		for m in monitor.iter() {
			fixture.enqueue(m.payout_probe());
		}

		for m in monitor.iter().take(4) {
			assert!(m.is_dropped())
		}

		for m in monitor.iter().skip(4) {
			assert!(!m.is_dropped())
		}
	}
}

#[cfg(test)]
mod alt_tests {
	use crate::circular_buffer::CircularBuffer;
	use crate::fixed::index_coordinator_test::IndexCoordinatorTestExtensions;
	use crate::fixed::{Buffer, Pow2IndexCoordinator};
	use crate::index_coordinator::IndexCoordinator;
	use crate::test_shared::{Monitor, MonitorGenerator, Probe};
	use std::assert_matches;
	use std::cell::Cell;
	use std::mem::MaybeUninit;
	use std::ops::Index;
	use std::ops::IndexMut;
	use std::panic::{catch_unwind, set_hook, take_hook, AssertUnwindSafe};
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

			assert_eq!(target.capacity(), CAPACITY);
			assert_eq!(target.coordinator.capacity(), CAPACITY)
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

			assert_eq!(target.capacity(), CAPACITY);
			assert_eq!(target.coordinator.capacity(), CAPACITY)
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
}
