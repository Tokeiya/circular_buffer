use super::index_coordinator::IndexCoordinator;
use crate::Error;

#[derive(Clone, Debug)]
pub struct GeneralIndexCoordinator<const N: usize> {
	head: usize,
	len: usize,
}

impl<const N: usize> Default for GeneralIndexCoordinator<N> {
	fn default() -> Self {
		GeneralIndexCoordinator { head: 0, len: 0 }
	}
}

impl<const N: usize> IndexCoordinator<N> for GeneralIndexCoordinator<N> {
	fn head_index(&self) -> crate::Result<usize> {
		if self.is_empty() {
			Err(crate::Error::Empty)
		} else {
			Ok(self.head)
		}
	}

	fn tail_index(&self) -> crate::Result<usize> {
		if self.is_empty() {
			Err(crate::Error::Empty)
		} else {
			Ok((self.head + self.len - 1) % N)
		}
	}

	fn enqueue_index(&mut self) {
		if self.len < N {
			self.len += 1;
		} else {
			self.head = (self.head + 1) % N;
		}
	}

	fn dequeue_index(&mut self) -> crate::Result<()> {
		if self.is_empty() {
			Err(Error::Empty)
		} else {
			self.head = (self.head + 1) % N;
			self.len -= 1;
			Ok(())
		}
	}

	fn pop_index(&mut self) -> crate::Result<()> {
		match self.len.checked_sub(1) {
			Some(len) => {
				self.len = len;
				Ok(())
			}
			None => Err(Error::Empty),
		}
	}

	fn real_to_virtual(&self, idx: usize) -> crate::Result<usize> {
		if self.len <= idx {
			Err(Error::IndexOutOfRange {
				index: idx,
				len: self.len,
			})
		} else {
			Ok((idx + N - self.head) % N)
		}
	}

	fn virtual_to_real(&self, idx: usize) -> crate::Result<usize> {
		if self.len <= idx {
			Err(Error::IndexOutOfRange {
				index: idx,
				len: self.len,
			})
		} else {
			Ok((idx + self.head) % N)
		}
	}

	fn capacity(&self) -> usize {
		N
	}

	fn len(&self) -> usize {
		self.len
	}
}

#[cfg(test)]
mod test {
	use super::super::index_coordinator_test as tests;
	use super::*;

	const CAPACITY: usize = 10;
	type Fixture = GeneralIndexCoordinator<CAPACITY>;

	impl tests::IndexCoordinatorTestExtensions<CAPACITY> for GeneralIndexCoordinator<CAPACITY> {
		fn mut_len(&mut self) -> &mut usize {
			&mut self.len
		}

		fn mut_head(&mut self) -> &mut usize {
			&mut self.head
		}

		fn fixture() -> Self {
			Self { head: 0, len: 0 }
		}
	}

	#[test]
	fn default() {
		tests::default::<CAPACITY, Fixture>();
	}

	#[test]
	fn head_index() {
		tests::head_index::<CAPACITY, Fixture>();
	}

	#[test]
	fn tail_index() {
		tests::tail_index::<CAPACITY, Fixture>();
	}

	#[test]
	fn enqueue_index() {
		tests::enqueue_index::<CAPACITY, Fixture>();
	}

	#[test]
	fn dequeue_index() {
		tests::dequeue_index::<CAPACITY, Fixture>()
	}

	#[test]
	fn pop_index() {
		tests::pop_index::<CAPACITY, Fixture>();
	}

	#[test]
	fn real_to_virtual() {
		tests::real_to_virtual::<CAPACITY, Fixture>();
	}

	#[test]
	fn virtual_to_real() {
		tests::virtual_to_real::<CAPACITY, Fixture>();
	}

	#[test]
	fn capacity() {
		tests::capacity::<CAPACITY, Fixture>();
	}

	#[test]
	fn len() {
		tests::len::<CAPACITY, Fixture>();
	}

	#[test]
	fn is_empty() {
		tests::is_empty::<CAPACITY, Fixture>()
	}

	#[test]
	fn is_full() {
		tests::is_full::<CAPACITY, Fixture>()
	}
}
