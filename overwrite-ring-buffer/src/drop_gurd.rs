use crate::IndexCoordinator;
use std::mem::MaybeUninit;
pub(crate) struct DropGuard<'a, T, C: IndexCoordinator> {
	storage: &'a mut [MaybeUninit<T>],
	coordinator: C,
}

impl<'a, T: 'a, C: IndexCoordinator> DropGuard<'a, T, C> {
	pub(crate) fn new(storage: &'a mut [MaybeUninit<T>], coordinator: C) -> Self {
		Self {
			storage,
			coordinator,
		}
	}

	pub(crate) fn drop_next(&mut self) -> bool {
		if let Ok(idx) = self.coordinator.head_index() {
			_ = self.coordinator.dequeue_index();
			unsafe {
				self.storage[idx].assume_init_drop();
			}
			true
		} else {
			false
		}
	}
}

impl<'a, T: 'a, C: IndexCoordinator> Drop for DropGuard<'a, T, C> {
	fn drop(&mut self) {
		while self.drop_next() {}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::fixed::index_coordinator_test::IndexCoordinatorTestExtensions;
	use crate::fixed::Pow2IndexCoordinator;
	use crate::test_shared::*;
	
	const SIZE: usize = 8;
	type Coordinator = Pow2IndexCoordinator<SIZE>;

	#[test]
	fn new() {
		let mut generator = MonitorGenerator::default();
		let monitor: [Monitor; SIZE] = std::array::from_fn(|_| generator.generate(false));
		let mut sample: [_; SIZE] =
			std::array::from_fn(|i| MaybeUninit::new(monitor[i].payout_probe()));
		let mut coordinator = Coordinator::default();

		*coordinator.mut_len() = SIZE;

		let expected = sample.as_ptr();

		let mut fixture = DropGuard::new(&mut sample, coordinator.clone());
		assert_eq!(fixture.storage.as_ptr(), expected);
		assert_eq!(fixture.coordinator.mut_len(), coordinator.mut_len());
		assert_eq!(fixture.coordinator.mut_head(), coordinator.mut_head());
	}

	#[test]
	fn drop_next() {
		let mut generator = MonitorGenerator::default();
		let monitor: [Monitor; SIZE] = std::array::from_fn(|_| generator.generate(false));
		let mut sample: [_; SIZE] =
			std::array::from_fn(|i| MaybeUninit::new(monitor[i].payout_probe()));
		let mut coordinator = Coordinator::default();

		*coordinator.mut_len() = SIZE;

		let mut fixture = DropGuard::new(&mut sample, coordinator.clone());
		while fixture.drop_next() {}

		assert!(monitor.iter().all(|m| m.is_dropped()));
	}

	#[test]
	fn full_drop() {
		let mut generator = MonitorGenerator::default();
		let monitor: [Monitor; SIZE] = std::array::from_fn(|_| generator.generate(false));
		let mut sample: [_; SIZE] =
			std::array::from_fn(|i| MaybeUninit::new(monitor[i].payout_probe()));
		let mut coordinator = Coordinator::default();

		*coordinator.mut_len() = SIZE;
		let fixture = DropGuard::new(&mut sample, coordinator.clone());
		drop(fixture);

		assert!(monitor.iter().all(|m| m.is_dropped()));
	}

	#[test]
	fn partial_drop() {
		let mut generator = MonitorGenerator::default();
		let monitor: [Monitor; SIZE] = std::array::from_fn(|_| generator.generate(false));
		let mut sample: [_; SIZE] =
			std::array::from_fn(|i| MaybeUninit::new(monitor[i].payout_probe()));
		let mut coordinator = Coordinator::default();

		*coordinator.mut_len() = SIZE;
		let mut fixture = DropGuard::new(&mut sample, coordinator.clone());

		for _ in 0..4 {
			fixture.drop_next();
		}

		for i in 0..4 {
			assert!(monitor[i].is_dropped());
		}

		for i in 4..SIZE {
			assert!(!monitor[i].is_dropped());
		}

		drop(fixture);

		assert!(monitor.iter().all(|m| m.is_dropped()));
	}
}
