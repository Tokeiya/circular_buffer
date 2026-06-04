use super::super::error::*;

pub trait ResizableIndexCoordinator: Clone + Default {
	fn head_index(&self) -> Result<usize>;
	fn tail_index(&self) -> Result<usize>;
	fn enqueue_index(&mut self);
	fn dequeue_index(&mut self) -> Result<()>;
	fn pop_index(&mut self) -> Result<()>;
	fn real_to_virtual(&self, idx: usize) -> Result<usize>;
	fn virtual_to_real(&self, idx: usize) -> Result<usize>;
	fn capacity(&self) -> usize;
	fn len(&self) -> usize;

	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	fn is_full(&self) -> bool {
		self.len() == self.capacity()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Default)]
	struct Dummy {
		pub len: usize,
		pub capacity: usize,
	}

	impl Clone for Dummy {
		fn clone(&self) -> Self {
			unimplemented!()
		}
	}

	impl ResizableIndexCoordinator for Dummy {
		fn head_index(&self) -> Result<usize> {
			unimplemented!()
		}

		fn tail_index(&self) -> Result<usize> {
			unimplemented!()
		}

		fn enqueue_index(&mut self) {
			unimplemented!()
		}

		fn dequeue_index(&mut self) -> Result<()> {
			unimplemented!()
		}

		fn pop_index(&mut self) -> Result<()> {
			unimplemented!()
		}

		fn real_to_virtual(&self, _: usize) -> Result<usize> {
			unimplemented!()
		}

		fn virtual_to_real(&self, _: usize) -> Result<usize> {
			unimplemented!()
		}

		fn capacity(&self) -> usize {
			self.capacity
		}

		fn len(&self) -> usize {
			self.len
		}
	}

	#[test]
	fn is_empty() {
		let mut dummy = Dummy::default();
		assert_eq!(dummy.len(), 0);
		dummy.capacity = 120;
		assert_eq!(dummy.capacity(), 120);
		assert!(dummy.is_empty());

		for i in 1..100 {
			dummy.len = i;
			assert_eq!(dummy.len(), i);
			assert!(!dummy.is_empty());
		}
	}

	#[test]
	fn is_full() {
		let dummy = Dummy::default();
		assert_eq!(dummy.len, 0);
		assert_eq!(dummy.capacity(), 0);
		assert!(dummy.is_full());
		assert!(dummy.is_empty());
	}
}
