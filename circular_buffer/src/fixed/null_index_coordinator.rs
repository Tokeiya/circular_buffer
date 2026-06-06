use super::IndexCoordinator;
use crate::error::*;

#[derive(Debug, Clone, Default)]
pub struct NullIndexCoordinator;

impl IndexCoordinator<0> for NullIndexCoordinator {
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

	fn len(&self) -> usize {
		todo!()
	}
}
