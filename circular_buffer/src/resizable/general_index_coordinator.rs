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
		Self {
			capacity,
			head: 0,
			len: 0,
		}
	}
}

impl IndexCoordinator for GeneralIndexCoordinator {
	//noinspection DuplicatedCode
	fn head_index(&self) -> Result<usize> {
		if self.len == 0 {
			Err(Error::Empty)
		} else {
			Ok(self.head)
		}
	}

	fn tail_index(&self) -> Result<usize> {
		if self.is_empty() {
			Err(crate::Error::Empty)
		} else {
			Ok((self.head + self.len - 1) % self.capacity)
		}
	}

	fn enqueue_index(&mut self) {
		if self.len < self.capacity {
			self.len += 1;
		} else {
			self.head = (self.head + 1) % self.capacity;
		}
	}

	fn dequeue_index(&mut self) -> Result<()> {
		if self.is_empty() {
			Err(Error::Empty)
		} else {
			self.head = (self.head + 1) % self.capacity;
			self.len -= 1;
			Ok(())
		}
	}

	//noinspection DuplicatedCode
	fn pop_index(&mut self) -> Result<()> {
		match self.len.checked_sub(1) {
			Some(len) => {
				self.len = len;
				Ok(())
			}
			None => Err(Error::Empty),
		}
	}

	fn real_to_virtual(&self, idx: usize) -> Result<usize> {
		if self.len <= idx {
			Err(Error::IndexOutOfRange {
				index: idx,
				len: self.len,
			})
		} else {
			Ok((idx + self.capacity - self.head) % self.capacity)
		}
	}

	fn virtual_to_real(&self, idx: usize) -> Result<usize> {
		if self.len <= idx {
			Err(Error::IndexOutOfRange {
				index: idx,
				len: self.len,
			})
		} else {
			Ok((idx + self.head) % self.capacity)
		}
	}

	fn capacity(&self) -> usize {
		self.capacity
	}

	fn len(&self) -> usize {
		self.len
	}
}

#[cfg(test)]
mod test {
	use super::super::index_coordinator_tests::IndexCoordinatorTestExtension;
	use super::*;

	const CAPACITY: usize = 10;
	type Fixture = GeneralIndexCoordinator;

	impl IndexCoordinatorTestExtension for GeneralIndexCoordinator {
		fn fixture(capacity: usize) -> Self {
			Self {
				capacity,
				head: 0,
				len: 0,
			}
		}

		fn mut_capacity(&mut self) -> &mut usize {
			&mut self.capacity
		}

		fn mut_head(&mut self) -> &mut usize {
			&mut self.head
		}

		fn mut_len(&mut self) -> &mut usize {
			&mut self.len
		}
	}

	#[test]
	fn new() {
		let fixture = GeneralIndexCoordinator::new(10);
		assert_eq!(fixture.capacity(), 10);
		assert_eq!(fixture.head, 0);
		assert_eq!(fixture.len, 0);
	}

	#[test]
	fn head_index() {
		Fixture::head_index_test(CAPACITY);
	}

	#[test]
	fn tail_index() {
		Fixture::tail_index_test(CAPACITY);
	}

	#[test]
	fn enqueue_index() {
		Fixture::enqueue_index_test(CAPACITY);
	}

	#[test]
	fn dequeue_index() {
		Fixture::dequeue_index_test(CAPACITY);
	}

	#[test]
	fn pop_index() {
		Fixture::pop_index_test(CAPACITY);
	}

	#[test]
	fn real_to_virtual() {
		Fixture::real_to_virtual_test(CAPACITY);
	}

	#[test]
	fn virtual_to_real() {
		Fixture::virtual_to_real_test(CAPACITY);
	}

	#[test]
	fn capacity() {
		Fixture::capacity_test(CAPACITY);
	}

	#[test]
	fn len() {
		Fixture::len_test(CAPACITY)
	}

	#[test]
	fn is_empty() {
		Fixture::is_empty_test(CAPACITY)
	}

	#[test]
	fn is_full() {
		Fixture::is_full_test(CAPACITY)
	}

	#[test]
	fn clone() {
		Fixture::clone_test(CAPACITY);
	}
}
