use super::iter::Iter;
use super::iter_mut::IterMut;
use crate::CircularBuffer;
use crate::resizable::IndexCoordinator;
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};
#[cfg(test)]
use std::{cell::Cell, rc::Rc};

pub struct Buffer<T, C: IndexCoordinator> {
	#[cfg(test)]
	probe: Option<Rc<Cell<usize>>>,
	pub(super) storage: Vec<MaybeUninit<T>>,
	pub(super) coordinator: C,
}

impl<T, C: IndexCoordinator> Buffer<T, C> {
	pub fn new(coordinator: C) -> Self {
		todo!()
	}
}

impl<T, C: IndexCoordinator> Index<usize> for Buffer<T, C> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		todo!()
	}
}

impl<T, C: IndexCoordinator> IndexMut<usize> for Buffer<T, C> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		todo!()
	}
}

impl<T, C: IndexCoordinator> CircularBuffer<T> for Buffer<T, C> {
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
		todo!()
	}

	fn enqueue(&mut self, item: T) {
		todo!()
	}

	fn dequeue(&mut self) -> Option<T> {
		todo!()
	}

	fn iter(&self) -> Self::Iter<'_> {
		todo!()
	}

	fn iter_mut(&mut self) -> Self::MutIter<'_> {
		todo!()
	}

	fn len(&self) -> usize {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::error::*;
	use crate::resizable::Pow2IndexCoordinator;
	use crate::resizable::index_coordinator_tests::IndexCoordinatorTestExtension;
	use crate::test_shared::{Monitor, MonitorGenerator, Probe};
	use std::mem::MaybeUninit;
	use std::panic::{AssertUnwindSafe, catch_unwind};

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

	fn create_monitor(size: usize) -> Vec<Monitor> {
		let mut factory = MonitorGenerator::default();
		(0..size).map(|_| factory.generate()).collect()
	}

	#[test]
	fn new() {
		let mut fixture = ProbeFixture::new(Pow2IndexCoordinator::try_new(CAPACITY).unwrap());
		assert_eq!(fixture.coordinator.ref_capacity(), &CAPACITY);
		assert_eq!(fixture.coordinator.mut_head(), &mut 0usize);
		assert_eq!(fixture.coordinator.mut_len(), &mut 0usize);
	}

	#[test]
	fn index() {
		let mut fixture = usize_fixture();
		catch_unwind(AssertUnwindSafe(|| _ = fixture[0])).unwrap_err();

		fn check(target: &UsizeFixture, n: usize, offset: usize) {
			for idx in 0..n {
				assert_eq!(target[idx], idx + offset);
			}

			catch_unwind(AssertUnwindSafe(|| {
				_ = target[n];
			}))
			.unwrap_err();
		}

		for i in 1..=CAPACITY {
			fixture.enqueue(i);
			check(&fixture, i, 1);
		}

		for i in 1..=CAPACITY {
			fixture.enqueue(i + CAPACITY);
			check(&fixture, i, i + 1);
		}
	}

	#[test]
	fn index_mut() {
		let mut fixture = probe_fixture();
		let mut generator=MonitorGenerator::default();
		let mut vec=(0..10).map(|_|generator.generate()).collect::<Vec<_>>();
		let v=&vec[0];
		
	}
}
