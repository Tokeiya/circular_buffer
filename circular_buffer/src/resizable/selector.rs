use super::GeneralIndexCoordinator;
use super::IndexCoordinator;
use super::Pow2IndexCoordinator;
use crate::error::*;

#[derive(Clone, Debug)]
pub enum CoordinatorSelector {
	General(GeneralIndexCoordinator),
	Pow2(Pow2IndexCoordinator),
}

impl CoordinatorSelector {
	pub fn new(capacity: usize) -> Result<Self> {
		if capacity == 0 {
			Err(Error::ZeroCapacity)
		} else if capacity.is_power_of_two() {
			Ok(CoordinatorSelector::Pow2(Pow2IndexCoordinator::new(
				capacity,
			)?))
		} else {
			Ok(CoordinatorSelector::General(GeneralIndexCoordinator::new(
				capacity,
			)))
		}
	}
}

impl IndexCoordinator for CoordinatorSelector {
	fn head_index(&self) -> Result<usize> {
		match self {
			CoordinatorSelector::General(g) => g.head_index(),
			CoordinatorSelector::Pow2(p) => p.head_index(),
		}
	}

	fn tail_index(&self) -> Result<usize> {
		match self {
			CoordinatorSelector::General(g) => g.tail_index(),
			CoordinatorSelector::Pow2(p) => p.tail_index(),
		}
	}

	fn enqueue_index(&mut self) {
		match self {
			CoordinatorSelector::General(g) => g.enqueue_index(),
			CoordinatorSelector::Pow2(p) => p.enqueue_index(),
		}
	}

	fn dequeue_index(&mut self) -> Result<()> {
		match self {
			CoordinatorSelector::General(g) => g.dequeue_index(),
			CoordinatorSelector::Pow2(p) => p.dequeue_index(),
		}
	}

	fn pop_index(&mut self) -> Result<()> {
		match self {
			CoordinatorSelector::General(g) => g.pop_index(),
			CoordinatorSelector::Pow2(p) => p.pop_index(),
		}
	}

	fn real_to_virtual(&self, idx: usize) -> Result<usize> {
		match self {
			CoordinatorSelector::General(g) => g.real_to_virtual(idx),
			CoordinatorSelector::Pow2(p) => p.real_to_virtual(idx),
		}
	}

	fn virtual_to_real(&self, idx: usize) -> Result<usize> {
		match self {
			CoordinatorSelector::General(g) => g.virtual_to_real(idx),
			CoordinatorSelector::Pow2(p) => p.virtual_to_real(idx),
		}
	}

	fn capacity(&self) -> usize {
		match self {
			CoordinatorSelector::General(g) => g.capacity(),
			CoordinatorSelector::Pow2(p) => p.capacity(),
		}
	}

	fn len(&self) -> usize {
		match self {
			CoordinatorSelector::General(g) => g.len(),
			CoordinatorSelector::Pow2(p) => p.len(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::super::general_index_coordinator::ext_impl;
	use super::super::index_coordinator_tests::IndexCoordinatorTestExtension;
	use super::*;
	use std::assert_matches;

	#[test]
	fn new() {
		assert_matches!(CoordinatorSelector::new(0), Err(Error::ZeroCapacity));
		assert_matches!(
			CoordinatorSelector::new(1),
			Ok(CoordinatorSelector::Pow2(_))
		);
		assert_matches!(
			CoordinatorSelector::new(2),
			Ok(CoordinatorSelector::Pow2(_))
		);

		assert_matches!(
			CoordinatorSelector::new(3),
			Ok(CoordinatorSelector::General(_))
		);
	}

	type Fixture = CoordinatorSelector;
	const POW2: usize = 64;
	const NON_POW2: usize = 65;

	impl IndexCoordinatorTestExtension for CoordinatorSelector {
		fn fixture(capacity: usize) -> Self {
			Fixture::new(capacity).unwrap()
		}

		fn ref_capacity(&self) -> &usize {
			match self {
				CoordinatorSelector::General(g) => g.ref_capacity(),
				CoordinatorSelector::Pow2(p) => p.ref_capacity(),
			}
		}

		fn mut_head(&mut self) -> &mut usize {
			match self {
				CoordinatorSelector::General(g) => g.mut_head(),
				CoordinatorSelector::Pow2(p) => p.mut_head(),
			}
		}

		fn mut_len(&mut self) -> &mut usize {
			match self {
				CoordinatorSelector::General(g) => g.mut_len(),
				CoordinatorSelector::Pow2(p) => p.mut_len(),
			}
		}
	}

	#[test]
	fn head_index() {
		Fixture::head_index_test(POW2);
		Fixture::head_index_test(NON_POW2);
	}

	#[test]
	fn tail_index() {
		Fixture::tail_index_test(POW2);
		Fixture::tail_index_test(NON_POW2);
	}

	#[test]
	fn enqueue_index() {
		Fixture::enqueue_index_test(POW2);
		Fixture::enqueue_index_test(NON_POW2);
	}

	#[test]
	fn dequeue_index() {
		Fixture::dequeue_index_test(POW2);
		Fixture::dequeue_index_test(NON_POW2);
	}

	#[test]
	fn pop_index() {
		Fixture::pop_index_test(POW2);
		Fixture::pop_index_test(NON_POW2);
	}

	#[test]
	fn real_to_virtual() {
		Fixture::real_to_virtual_test(POW2);
		Fixture::real_to_virtual_test(NON_POW2);
	}

	#[test]
	fn virtual_to_real() {
		Fixture::virtual_to_real_test(POW2);
		Fixture::virtual_to_real_test(NON_POW2);
	}

	#[test]
	fn capacity() {
		Fixture::capacity_test(POW2);
		Fixture::capacity_test(NON_POW2);
	}

	#[test]
	fn len() {
		Fixture::len_test(POW2);
		Fixture::len_test(NON_POW2);
	}

	#[test]
	fn is_empty() {
		Fixture::is_empty_test(POW2);
		Fixture::is_empty_test(NON_POW2);
	}

	#[test]
	fn is_full() {
		Fixture::is_full_test(POW2);
		Fixture::is_full_test(NON_POW2);
	}

	#[test]
	fn clone() {
		Fixture::clone_test(POW2);
		Fixture::clone_test(NON_POW2);
	}
}
