use super::super::error::*;
use crate::index_coordinator::IndexCoordinator;

pub trait FixedIndexCoordinator<const N: usize>: IndexCoordinator + Default {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::fixed::index_coordinator_test as tests;
	use crate::index_coordinator::IndexCoordinator;

	#[derive(Default, Clone)]
	struct Dummy<const N: usize> {
		pub len: usize,
	}

	impl<const N: usize> IndexCoordinator for Dummy<N> {
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
			unimplemented!()
		}

		fn len(&self) -> usize {
			self.len
		}
	}

	impl<const N: usize> FixedIndexCoordinator<N> for Dummy<N> {}

	impl<const N: usize> tests::IndexCoordinatorTestExtensions<N> for Dummy<N> {
		fn mut_len(&mut self) -> &mut usize {
			&mut self.len
		}

		fn mut_head(&mut self) -> &mut usize {
			unimplemented!()
		}

		fn fixture() -> Self {
			Dummy::<N> { len: 0 }
		}
	}

	#[test]
	fn is_empty() {
		tests::is_empty::<10, Dummy<10>>()
	}

	#[test]
	fn is_full() {
		tests::is_full::<10, Dummy<10>>()
	}
}
