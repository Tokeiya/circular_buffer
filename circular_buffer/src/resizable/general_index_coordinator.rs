use super::index_coordinator::IndexCoordinator;
use crate::error::*;

#[derive(Clone, Debug)]
pub struct GeneralIndexCoordinator {
	capacity: usize,
	head: usize,
	len: usize,
}

impl GeneralIndexCoordinator {
	pub fn new(capacity: usize) -> Self {
		todo!()
	}
}

impl IndexCoordinator for GeneralIndexCoordinator {
	fn head_index(&self) -> Result<usize> {
		todo!()
	}

	fn tail_index(&self) -> Result<usize> {
		todo!()
	}

	fn enqueue_index(&mut self) {
		todo!()
	}

	fn dequeue_index(&mut self) -> Result<()> {
		todo!()
	}

	fn pop_index(&mut self) -> Result<()> {
		todo!()
	}

	fn real_to_virtual(&self, idx: usize) -> Result<usize> {
		todo!()
	}

	fn virtual_to_real(&self, idx: usize) -> Result<usize> {
		todo!()
	}

	fn capacity(&self) -> usize {
		todo!()
	}

	fn len(&self) -> usize {
		todo!()
	}
}
