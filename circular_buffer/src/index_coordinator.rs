use super::error::*;

pub trait IndexCoordinator {
	fn head_index(&self) -> Result<usize>;
	fn tail_index(&self) -> Result<usize>;
	fn enqueue_index(&mut self);
	fn dequeue_index(&mut self) -> Result<()>;
	fn pop_index(&mut self) -> Result<()>;
	fn real_to_virtual(&self, idx: usize) -> Result<usize>;
	fn virtual_to_real(&self, idx: usize) -> Result<usize>;
	fn capacity(&self) -> usize;
	fn len(&self) -> usize;
}
