#[cfg(test)]
use std::{cell::Cell, rc::Rc};

use super::IndexCoordinator;
use super::iter::Iter;
use super::iter_mut::IterMut;
use crate::circular_buffer::CircularBuffer;
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

pub struct Buffer<T, C: IndexCoordinator<N>, const N: usize> {
	#[cfg(test)]
	probe: Option<Rc<Cell<usize>>>,
	pub(super) storage: [MaybeUninit<T>; N],
	pub(super) coordinator: C,
}

impl<T, C: IndexCoordinator<N>, const N: usize> Default for Buffer<T, C, N> {
	fn default() -> Self {
		Self {
			storage: [const { MaybeUninit::uninit() }; N],
			coordinator: C::default(),
			#[cfg(test)]
			probe: None,
		}
	}
}

impl<T, C: IndexCoordinator<N>, const N: usize> Index<usize> for Buffer<T, C, N> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		match self.coordinator.virtual_to_real(index) {
			Ok(i) => unsafe { self.storage[i].assume_init_ref() },
			Err(_) => panic!("Index out of bounds"),
		}
	}
}

impl<T, C: IndexCoordinator<N>, const N: usize> IndexMut<usize> for Buffer<T, C, N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match self.coordinator.virtual_to_real(index) {
			Ok(i) => unsafe { self.storage[i].assume_init_mut() },
			Err(_) => panic!("Index out of bounds"),
		}
	}
}

impl<T, C: IndexCoordinator<N>, const N: usize> CircularBuffer<T> for Buffer<T, C, N> {
	type Iter<'a>
		= Iter<'a, T, C, N>
	where
		T: 'a,
		Self: 'a;

	type MutIter<'a>
		= IterMut<'a, T, C, N>
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
			unsafe {
				self.storage[index].assume_init_drop();
				self.storage[index].write(item);
			};

			self.coordinator.enqueue_index();
		}
	}

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
		Iter::new(self)
	}

	fn iter_mut(&mut self) -> Self::MutIter<'_> {
		Self::MutIter::new(self)
	}

	fn len(&self) -> usize {
		self.coordinator.len()
	}
}

impl<T, C: IndexCoordinator<N>, const N: usize> Buffer<T, C, N> {
	#[cfg(test)]
	fn new_with_probe(probe: Rc<Cell<usize>>) -> Self {
		let mut s = Self::default();
		s.probe = Some(probe);
		s
	}
	pub(super) fn get_raw(&self, real_index: usize) -> &T {
		unsafe { self.storage[real_index].assume_init_ref() }
	}
}

impl<T, C: IndexCoordinator<N>, const N: usize> Drop for Buffer<T, C, N> {
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
	use crate::fixed::Pow2IndexCoordinator;
	use crate::test_shared::{Monitor, MonitorGenerator, Probe};
	use std::cell::Cell;
	use std::panic::{AssertUnwindSafe, catch_unwind};
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
