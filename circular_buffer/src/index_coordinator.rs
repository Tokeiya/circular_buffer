//IndexCoordinatorを統一化することで、Resizable/FixedのIterとIterMutを統一できないか検討してみる
pub trait IndexCoordinator: Clone {
	fn head_index(&self) -> crate::Result<usize>;
	fn tail_index(&self) -> crate::Result<usize>;
	fn enqueue_index(&mut self);
	fn dequeue_index(&mut self) -> crate::Result<()>;
	fn pop_index(&mut self) -> crate::Result<()>;
	fn virtual_to_real(&self, idx: usize) -> crate::Result<usize>;
	fn capacity(&self) -> usize;
	fn len(&self) -> usize;

	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	fn is_full(&self) -> bool {
		self.len() == self.capacity()
	}
}
