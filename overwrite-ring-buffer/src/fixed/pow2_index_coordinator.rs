use crate::error::*;
use crate::fixed::index_coordinator::FixedIndexCoordinator;
use crate::index_coordinator::IndexCoordinator;
use crate::index_coordinator::sealed::Sealed;

#[derive(Clone)]
pub struct Pow2IndexCoordinator<const N: usize> {
	head: usize,
	len: usize,
}

impl<const N: usize> Pow2IndexCoordinator<N> {
	const CHECK: () = assert!(N.count_ones() == 1 && N != 0);
	const MASK: usize = N - 1;

	#[allow(clippy::let_unit_value)]
	fn new() -> Self {
		_ = Self::CHECK;
		Self { head: 0, len: 0 }
	}
}

impl<const N: usize> Default for Pow2IndexCoordinator<N> {
	fn default() -> Self {
		Self::new()
	}
}

impl<const N: usize> Sealed for Pow2IndexCoordinator<N> {}

impl<const N: usize> IndexCoordinator for Pow2IndexCoordinator<N> {
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
		if self.len == 0 {
			Err(Error::Empty)
		} else {
			Ok((self.head + self.len - 1) & Self::MASK)
		}
	}

	#[inline]
	fn enqueue_index(&mut self) {
		if self.len < N {
			self.len += 1;
		} else {
			self.head = (self.head + 1) & Self::MASK;
		}
	}

	#[inline]
	fn dequeue_index(&mut self) -> Result<()> {
		if self.len == 0 {
			Err(Error::Empty)
		} else {
			self.head = (self.head + 1) & Self::MASK;
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
			Ok((idx + self.head) & Self::MASK)
		}
	}

	#[inline]
	fn capacity(&self) -> usize {
		N
	}

	#[inline]
	fn len(&self) -> usize {
		self.len
	}
}

impl<const N: usize> FixedIndexCoordinator<N> for Pow2IndexCoordinator<N> {}

#[cfg(test)]
mod tests {
	use super::super::index_coordinator_test as tests;
	use super::*;
	use crate::fixed::index_coordinator_test::IndexCoordinatorTestExtensions;
	impl<const N: usize> IndexCoordinatorTestExtensions<N> for Pow2IndexCoordinator<N> {
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

	const BASE: usize = 8;

	type Fixture = Pow2IndexCoordinator<BASE>;

	#[test]
	fn new() {
		let fixture = Fixture::new();
		assert_eq!(fixture.head, 0);
		assert_eq!(fixture.len, 0);
	}

	#[test]
	fn default() {
		tests::default::<BASE, Fixture>();
	}

	#[test]
	fn head_index() {
		tests::head_index::<BASE, Fixture>()
	}

	#[test]
	fn tail_index() {
		tests::tail_index::<BASE, Fixture>()
	}

	#[test]
	fn enqueue_index() {
		tests::enqueue_index::<BASE, Fixture>();
	}

	#[test]
	fn dequeue_index() {
		tests::dequeue_index::<BASE, Fixture>();
	}

	#[test]
	fn pop_index() {
		tests::pop_index::<BASE, Fixture>();
	}

	#[test]
	fn virtual_to_real() {
		tests::resolve_index::<BASE, Fixture>();
	}

	#[test]
	fn capacity() {
		tests::capacity::<BASE, Fixture>();
	}

	#[test]
	fn len() {
		tests::len::<BASE, Fixture>();
	}

	#[test]
	fn is_empty() {
		tests::is_empty::<BASE, Fixture>();
	}

	#[test]
	fn is_full() {
		tests::is_full::<BASE, Fixture>();
	}

	#[test]
	fn clone() {
		tests::clone::<BASE, Fixture>();
	}
}
