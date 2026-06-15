use super::index_coordinator::ResizableIndexCoordinator;
use crate::error::*;
use crate::index_coordinator::sealed::Sealed;
use crate::index_coordinator::IndexCoordinator;

#[derive(Clone, Debug)]
pub struct GeneralIndexCoordinator {
	capacity: usize,
	head: usize,
	len: usize,
}

impl GeneralIndexCoordinator {
	pub fn try_new(capacity: usize) -> Result<Self> {
		if capacity == 0 {
			Err(Error::ZeroCapacity)
		} else {
			Ok(Self {
				capacity,
				head: 0,
				len: 0,
			})
		}
	}
}

impl Sealed for GeneralIndexCoordinator {}

impl IndexCoordinator for GeneralIndexCoordinator {
	#[inline]
	//noinspection DuplicatedCode
	fn head_index(&self) -> Result<usize> {
		if self.len == 0 {
			Err(Error::Empty)
		} else {
			Ok(self.head)
		}
	}

	#[inline]
	fn tail_index(&self) -> Result<usize> {
		if self.is_empty() {
			Err(crate::Error::Empty)
		} else {
			Ok((self.head + self.len - 1) % self.capacity)
		}
	}

	#[inline]
	fn enqueue_index(&mut self) {
		if self.len < self.capacity {
			self.len += 1;
		} else {
			self.head = (self.head + 1) % self.capacity;
		}
	}

	#[inline]
	fn dequeue_index(&mut self) -> Result<()> {
		if self.is_empty() {
			Err(Error::Empty)
		} else {
			self.head = (self.head + 1) % self.capacity;
			self.len -= 1;
			Ok(())
		}
	}

	#[inline]
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

	#[inline]
	fn resolve_index(&self, idx: usize) -> Result<usize> {
		if self.len <= idx {
			Err(Error::IndexOutOfRange {
				index: idx,
				len: self.len,
			})
		} else {
			Ok((idx + self.head) % self.capacity)
		}
	}

	#[inline]
	fn capacity(&self) -> usize {
		self.capacity
	}

	#[inline]
	fn len(&self) -> usize {
		self.len
	}
}

impl ResizableIndexCoordinator for GeneralIndexCoordinator {
	fn empty_like(&self) -> Self {
		Self {
			capacity: self.capacity,
			len: 0,
			head: 0,
		}
	}
}

#[cfg(test)]
pub(crate) mod ext_impl {
	use crate::resizable::index_coordinator_tests::IndexCoordinatorTestExtension;
	use crate::resizable::GeneralIndexCoordinator;
	
	impl IndexCoordinatorTestExtension for GeneralIndexCoordinator {
		fn fixture(capacity: usize) -> Self {
			Self {
				capacity,
				head: 0,
				len: 0,
			}
		}

		fn ref_capacity(&self) -> &usize {
			&self.capacity
		}

		fn mut_head(&mut self) -> &mut usize {
			&mut self.head
		}

		fn mut_len(&mut self) -> &mut usize {
			&mut self.len
		}
	}
}

#[cfg(test)]
mod test {
	use super::super::index_coordinator_tests::IndexCoordinatorTestExtension;
	use super::*;
	use crate::IndexCoordinator;
	
	const CAPACITY: usize = 10;
	type Fixture = GeneralIndexCoordinator;

	#[test]
	fn new() {
		let fixture = GeneralIndexCoordinator::try_new(10).unwrap();
		assert_eq!(fixture.capacity(), 10);
		assert_eq!(fixture.head, 0);
		assert_eq!(fixture.len, 0);

		matches!(
			GeneralIndexCoordinator::try_new(0),
			Err(Error::ZeroCapacity)
		);
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
	fn virtual_to_real() {
		Fixture::resolve_index_test(CAPACITY);
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

	#[test]
	fn empty_like() {
		Fixture::empty_like_test(CAPACITY);
	}
}
