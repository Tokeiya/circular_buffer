use super::GeneralIndexCoordinator;
use super::Pow2IndexCoordinator;
use super::ResizableIndexCoordinator;
use crate::error::*;
use crate::index_coordinator::IndexCoordinator;

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
			Ok(CoordinatorSelector::Pow2(Pow2IndexCoordinator::try_new(
				capacity,
			)?))
		} else {
			Ok(CoordinatorSelector::General(
				GeneralIndexCoordinator::try_new(capacity)?,
			))
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
			CoordinatorSelector::Pow2(p) => p.dequeue_index(),
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

impl ResizableIndexCoordinator for CoordinatorSelector {}

#[cfg(test)]
mod tests {
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

	impl IndexCoordinatorTestExtension for CoordinatorSelector {
		fn fixture(capacity: usize) -> Self {
			Fixture::new(capacity).unwrap()
		}

		fn ref_capacity(&self) -> &usize {
			todo!()
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
}
