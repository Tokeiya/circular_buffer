use crate::error::*;

#[derive(Clone)]
pub struct IndexCoordinator<const N: usize> {
	head: usize,
	len: usize,
}

impl<const N: usize> IndexCoordinator<N> {
	const CHECK: () = assert!(N.count_ones() == 1);
	const MASK: usize = N - 1;

	#[allow(clippy::let_unit_value)]
	pub fn new() -> Self {
		_ = Self::CHECK;
		Self { head: 0, len: 0 }
	}

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
			Ok((self.head + self.len - 1) & Self::MASK)
		}
	}

	pub fn enqueue_index(&mut self) {
		if self.len < N {
			self.len += 1;
		} else {
			self.head = (self.head + 1) & Self::MASK;
		}
	}

	pub fn dequeue_index(&mut self) -> Result<()> {
		if self.len == 0 {
			Err(Error::Empty)
		} else {
			self.head = (self.head + 1) & Self::MASK;
			self.len -= 1;
			Ok(())
		}
	}

	pub fn pop_index(&mut self) -> Result<()> {
		match self.len.checked_sub(1) {
			Some(len) => {
				self.len = len;
				Ok(())
			}
			None => Err(Error::Empty),
		}
	}

	#[allow(dead_code)]
	pub fn real_to_virtual(&self, idx: usize) -> Result<usize> {
		if self.len <= idx {
			Err(Error::IndexOutOfRange {
				index: idx,
				len: self.len,
			})
		} else {
			Ok(idx.wrapping_sub(self.head) & Self::MASK)
		}
	}

	pub fn virtual_to_real(&self, idx: usize) -> Result<usize> {
		if self.len <= idx {
			Err(Error::IndexOutOfRange {
				index: idx,
				len: self.len,
			})
		} else {
			Ok((idx + self.head) & Self::MASK)
		}
	}

	#[allow(dead_code)]
	pub fn capacity(&self) -> usize {
		N
	}

	pub fn len(&self) -> usize {
		self.len
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::assert_matches;

	const BASE: usize = 8;
	const MASK: usize = BASE - 1;
	fn expected_real_to_virtual<const N: usize>(index: usize, head: usize) -> usize {
		(index + N - head) % N
	}

	fn expected_virtual_to_real<const N: usize>(index: usize, head: usize) -> usize {
		(index + head) % N
	}

	#[test]
	fn head_index() {
		let mut fixture = IndexCoordinator::<BASE>::new();
		assert_matches!(fixture.head_index(), Err(Error::Empty));

		fixture.head = 0;
		fixture.len = 8;

		assert_eq!(fixture.head_index().unwrap(), 0);
		fixture.pop_index().unwrap();
		assert_eq!(fixture.head_index().unwrap(), 0);
		fixture.dequeue_index().unwrap();
		assert_eq!(fixture.head_index().unwrap(), 1);
		fixture.enqueue_index();
		assert_eq!(fixture.head_index().unwrap(), 1);
	}

	#[test]
	fn tail_index() {
		let mut fixture = IndexCoordinator::<BASE>::new();
		assert_matches!(fixture.tail_index(), Err(Error::Empty));

		fixture.head = 0;
		fixture.len = 8;

		assert_eq!(fixture.tail_index().unwrap(), 7);
		fixture.pop_index().unwrap();
		assert_eq!(fixture.tail_index().unwrap(), 6);
		fixture.dequeue_index().unwrap();
		assert_eq!(fixture.tail_index().unwrap(), 6);
		fixture.enqueue_index();
		assert_eq!(fixture.tail_index().unwrap(), 7);
	}

	#[test]
	fn verify() {
		for (v, r) in (0..BASE).map(|i| (i, (i + 3) % BASE)) {
			assert_eq!(expected_real_to_virtual::<BASE>(r, 3), v);
			assert_eq!(expected_virtual_to_real::<BASE>(v, 3), r);
		}
	}

	#[test]
	fn new() {
		let fixture = IndexCoordinator::<BASE>::new();
		assert_eq!(fixture.head, 0);
		assert_eq!(fixture.len, 0);
	}

	#[test]
	fn capacity() {
		let mut fixture = IndexCoordinator::<BASE>::new();
		assert_eq!(fixture.capacity(), BASE);

		for _ in 0..100 {
			fixture.enqueue_index();
			assert_eq!(fixture.capacity(), BASE);
		}
	}

	#[test]
	fn enqueue_index() {
		let mut fixture = IndexCoordinator::<BASE>::new();

		for i in 0..BASE {
			fixture.enqueue_index();
			assert_eq!(fixture.len, i + 1);
			assert_eq!(fixture.head, 0);
		}

		for i in 0..BASE {
			fixture.enqueue_index();
			assert_eq!(fixture.len, BASE);
			assert_eq!(fixture.head, (i + 1) & MASK);
		}
	}

	#[test]
	fn dequeue_index() {
		let mut fixture = IndexCoordinator::<BASE>::new();
		assert!(matches!(fixture.dequeue_index(), Err(Error::Empty)));

		fixture.head = 0;
		fixture.len = BASE;

		for i in 0..BASE {
			assert_eq!(fixture.head, i);
			assert_eq!(fixture.len, BASE - i);
			fixture.dequeue_index().unwrap();
		}

		assert_eq!(fixture.head, 0);
		assert_eq!(fixture.len, 0);

		assert!(matches!(fixture.dequeue_index(), Err(Error::Empty)));
		assert_eq!(fixture.head, 0);
		assert_eq!(fixture.len, 0);

		const INIT: usize = 4;
		fixture.head = INIT;
		fixture.len = BASE;

		for i in 0..BASE {
			assert_eq!(fixture.len, BASE - i);
			assert_eq!(fixture.head, INIT.wrapping_add(i) & MASK);
			fixture.dequeue_index().unwrap();
		}
	}

	#[test]
	fn pop_index() {
		let mut fixture = IndexCoordinator::<BASE>::new();
		assert_matches!(fixture.pop_index(), Err(Error::Empty));

		fixture.head = 0;
		fixture.len = BASE;

		for i in 0..BASE {
			assert_eq!(fixture.len, BASE - i);
			assert_eq!(fixture.head, 0);
			assert_eq!(
				fixture.real_to_virtual(fixture.len - 1).unwrap(),
				expected_real_to_virtual::<BASE>(fixture.len - 1, 0)
			);
			fixture.pop_index().unwrap();
		}

		fixture.head = 4;
		fixture.len = BASE;

		for i in 0..BASE {
			assert_eq!(fixture.len, BASE - i);
			assert_eq!(fixture.head, 4);
			fixture.pop_index().unwrap();
		}
	}

	#[test]
	fn len() {
		let mut fixture = IndexCoordinator::<BASE>::new();

		for i in 0..BASE {
			assert_eq!(fixture.len(), i);
			fixture.enqueue_index();
			assert_eq!(fixture.len(), i + 1);
		}

		for _ in 0..BASE {
			assert_eq!(fixture.len(), BASE);
			fixture.enqueue_index();
			assert_eq!(fixture.len(), BASE);
		}
	}

	#[test]
	fn real_to_virtual() {
		let mut fixture = IndexCoordinator::<BASE>::new();

		for i in 0..BASE {
			for r in 0..i {
				let act = fixture.real_to_virtual(r).unwrap();
				assert_eq!(act, r);
			}

			for _ in i..BASE {
				assert!(matches!(
					fixture.real_to_virtual(i),
					Err(Error::IndexOutOfRange { index: _, len: _ })
				));
			}

			fixture.enqueue_index();
		}

		let mut expected = 1usize;

		for _ in 0..100 {
			fixture.enqueue_index();
			assert_eq!(fixture.head, expected);
			expected = (expected + 1) % BASE;

			for r in 0..BASE {
				assert_eq!(
					fixture.real_to_virtual(r).unwrap(),
					expected_real_to_virtual::<BASE>(r, fixture.head)
				);
			}
		}
	}

	#[test]
	fn virtual_to_real() {
		let mut fixture = IndexCoordinator::<BASE>::new();

		for i in 0..BASE {
			for v in 0..i {
				let act = fixture.virtual_to_real(v).unwrap();
				assert_eq!(act, v);
			}

			for _ in i..BASE {
				assert!(matches!(
					fixture.virtual_to_real(i),
					Err(Error::IndexOutOfRange { index: _, len: _ })
				));
			}

			fixture.enqueue_index();
		}

		let mut expected = 1usize;

		for _ in 0..100 {
			fixture.enqueue_index();
			assert_eq!(fixture.head, expected);
			expected = (expected + 1) % BASE;

			for v in 0..BASE {
				assert_eq!(
					fixture.virtual_to_real(v).unwrap(),
					expected_virtual_to_real::<BASE>(v, fixture.head)
				);
			}
		}
	}

	#[test]
	fn single() {
		let mut fixture = IndexCoordinator::<1>::new();

		for _ in 0..100 {
			fixture.enqueue_index();
			assert_eq!(fixture.len(), 1);
			assert_eq!(fixture.head, 0);
		}
	}

	#[test]
	fn complex() {
		let mut fixture = IndexCoordinator::<BASE>::new();

		fixture.enqueue_index();
		fixture.enqueue_index();
		fixture.dequeue_index().unwrap();

		assert_eq!(fixture.head, 1);
		assert_eq!(fixture.len, 1);

		fixture.enqueue_index();
		fixture.enqueue_index();
		fixture.enqueue_index();
		fixture.dequeue_index().unwrap();
		fixture.dequeue_index().unwrap();
		assert_eq!(fixture.len, 2);
		assert_eq!(fixture.head, 3);

		for _ in 0..6 {
			fixture.enqueue_index();
		}

		for _ in 0..8 {
			fixture.dequeue_index().unwrap();
		}

		assert_eq!(fixture.len, 0);
		assert_eq!(fixture.head, 3);
	}
}
