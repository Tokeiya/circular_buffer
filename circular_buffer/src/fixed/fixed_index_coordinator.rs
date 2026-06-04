use super::super::error::*;

pub trait FixedIndexCoordinator<const N: usize>: Clone + Default {
	fn head_index(&self) -> Result<usize>;
	fn tail_index(&self) -> Result<usize>;
	fn enqueue_index(&mut self);
	fn dequeue_index(&mut self) -> Result<()>;
	fn pop_index(&mut self) -> Result<()>;
	fn real_to_virtual(&self, idx: usize) -> Result<usize>;
	fn virtual_to_real(&self, idx: usize) -> Result<usize>;
	fn capacity(&self) -> usize {
		N
	}
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

	#[derive(Default, Clone)]
	struct Dummy<const N: usize> {
		pub len: usize,
	}

	impl<const N: usize> FixedIndexCoordinator<N> for Dummy<N> {
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

		fn len(&self) -> usize {
			self.len
		}
	}

	#[test]
	fn is_empty() {
		let mut fixture = Dummy::<10>::default();
		assert_eq!(fixture.len(), 0);
		assert!(fixture.is_empty());

		fixture.len = 10;
		assert_eq!(fixture.len(), 10);
		assert!(!fixture.is_empty());
	}

	#[test]
	fn is_full() {
		let mut fixture = Dummy::<10>::default();
		assert!(!fixture.is_full());

		fixture.len = 10;
		assert!(fixture.is_full());
	}
}
