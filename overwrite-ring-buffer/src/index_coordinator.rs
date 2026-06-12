pub(crate) mod sealed {
	pub trait Sealed {}
}

pub trait IndexCoordinator: Clone + sealed::Sealed {
	fn head_index(&self) -> crate::Result<usize>;
	fn tail_index(&self) -> crate::Result<usize>;
	fn enqueue_index(&mut self);
	fn dequeue_index(&mut self) -> crate::Result<()>;
	fn pop_index(&mut self) -> crate::Result<()>;
	fn resolve_index(&self, idx: usize) -> crate::Result<usize>;
	fn capacity(&self) -> usize;
	fn len(&self) -> usize;

	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	fn is_full(&self) -> bool {
		self.len() == self.capacity()
	}
}
