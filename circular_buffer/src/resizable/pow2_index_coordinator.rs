use crate::error::*;
use crate::resizable::index_coordinator::IndexCoordinator;
#[derive(Clone, Debug)]
pub struct Pow2IndexCoordinator {
	head: usize,
	len: usize,
	capacity: usize,
	mask: usize,
}

impl Pow2IndexCoordinator {
	pub fn try_new(capacity: usize) -> Result<Self> {
		if capacity == 0 {
			Err(Error::ZeroCapacity)
		} else if capacity.count_ones() != 1 {
			Err(Error::CapacityNotPow2(capacity))
		} else {
			Ok(Self {
				head: 0,
				len: 0,
				capacity,
				mask: capacity - 1,
			})
		}
	}
}

impl IndexCoordinator for Pow2IndexCoordinator {
	//noinspection DuplicatedCode
	fn head_index(&self) -> Result<usize> {
		if self.len == 0 {
			Err(Error::Empty)
		} else {
			Ok(self.head)
		}
	}

	fn tail_index(&self) -> Result<usize> {
		if self.len == 0 {
			Err(Error::Empty)
		} else {
			Ok((self.head + self.len - 1) & self.mask)
		}
	}

	fn enqueue_index(&mut self) {
		if self.len < self.capacity {
			self.len += 1;
		} else {
			self.head = (self.head + 1) & self.mask;
		}
	}

	fn dequeue_index(&mut self) -> Result<()> {
		if self.len == 0 {
			Err(Error::Empty)
		} else {
			self.head = (self.head + 1) & self.mask;
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
			Ok(idx.wrapping_sub(self.head) & self.mask)
		}
	}

	fn virtual_to_real(&self, idx: usize) -> Result<usize> {
		if self.len <= idx {
			Err(Error::IndexOutOfRange {
				index: idx,
				len: self.len,
			})
		} else {
			Ok((idx + self.head) & self.mask)
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
pub(crate) mod ext_impl {
	use crate::resizable::Pow2IndexCoordinator;
	use crate::resizable::index_coordinator_tests::IndexCoordinatorTestExtension;

	impl IndexCoordinatorTestExtension for Pow2IndexCoordinator {
		fn fixture(capacity: usize) -> Self {
			Pow2IndexCoordinator::try_new(capacity).unwrap()
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
mod tests {
	use super::super::index_coordinator_tests::IndexCoordinatorTestExtension;
	use super::*;
	use std::assert_matches;

	const CAPACITY: usize = 8;
	type Fixture = Pow2IndexCoordinator;

	#[test]
	fn new() {
		assert_matches!(Pow2IndexCoordinator::try_new(0), Err(Error::ZeroCapacity));

		let fixture = Pow2IndexCoordinator::try_new(8).unwrap();
		assert_eq!(fixture.head, 0);
		assert_eq!(fixture.len, 0);
		assert_eq!(fixture.capacity, 8);
		assert_eq!(fixture.mask, 7);
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
		Fixture::len_test(CAPACITY);
	}

	#[test]
	fn is_empty() {
		Fixture::is_empty_test(CAPACITY);
	}

	#[test]
	fn is_full() {
		Fixture::is_full_test(CAPACITY);
	}

	#[test]
	fn clone() {
		Fixture::clone_test(CAPACITY);
	}
}
