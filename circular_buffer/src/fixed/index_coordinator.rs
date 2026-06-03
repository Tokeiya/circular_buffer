use crate::Error;
use crate::index_coordinator::IndexCoordinator as TraitIndexCoordinator;

#[derive(Clone, Debug)]
pub struct IndexCoordinator<const N: usize> {
	head: usize,
	len: usize,
}

impl<const N: usize> Default for IndexCoordinator<N> {
	fn default() -> Self {
		IndexCoordinator { head: 0, len: 0 }
	}
}

impl<const N: usize> TraitIndexCoordinator for IndexCoordinator<N> {
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
	use super::*;
	use crate::error::*;
	use std::assert_matches;

	const CAPACITY: usize = 10;
	type Fixture = IndexCoordinator<CAPACITY>;

	fn expected_real_to_virtual<const N: usize>(index: usize, head: usize) -> usize {
		(index + N - head) % N
	}

	fn expected_virtual_to_real<const N: usize>(index: usize, head: usize) -> usize {
		(index + head) % N
	}

	fn fixture() -> Fixture {
		Fixture { head: 0, len: 0 }
	}

	#[test]
	fn default() {
		let fixture = Fixture::default();
		assert_eq!(fixture.head, 0);
		assert_eq!(fixture.len, 0);
	}

	#[test]
	fn head_index() {
		let mut fixture = fixture();
		assert_matches!(fixture.head_index(), Err(crate::Error::Empty));
		fixture.len = 1;

		for i in 0..CAPACITY {
			fixture.head = i;
			assert_eq!(fixture.head_index().unwrap(), i);
		}
	}

	#[test]
	fn tail_index() {
		let mut fixture = fixture();
		assert_matches!(fixture.tail_index(), Err(crate::Error::Empty));

		for h in 0..CAPACITY {
			for l in 1..=CAPACITY {
				fixture.len = l;
				fixture.head = h;
				assert_eq!(
					fixture.tail_index().unwrap(),
					expected_virtual_to_real::<CAPACITY>(l - 1, h)
				)
			}
		}
	}

	#[test]
	fn enqueue_index() {
		let mut fixture = fixture();

		for i in 0..CAPACITY {
			assert_eq!(fixture.len, i);
			assert_eq!(fixture.head, 0);
			fixture.enqueue_index();
		}

		for i in 0..CAPACITY {
			assert_eq!(fixture.len, CAPACITY);
			assert_eq!(fixture.head, i);
			fixture.enqueue_index();
		}
	}

	#[test]
	fn dequeue_index() {
		let mut fixture = fixture();
		assert_matches!(fixture.dequeue_index(), Err(crate::Error::Empty));

		fixture.len = CAPACITY;

		for i in 0..CAPACITY {
			assert_eq!(fixture.head, i);
			assert_eq!(fixture.len, CAPACITY - i);
			fixture.dequeue_index().unwrap();
		}

		assert_eq!(fixture.head, 0);
		assert_eq!(fixture.len, 0);

		assert_matches!(fixture.dequeue_index(), Err(crate::Error::Empty));
	}

	#[test]
	fn pop_index() {
		let mut fixture = fixture();
		assert_matches!(fixture.pop_index(), Err(crate::Error::Empty));

		fixture.len = CAPACITY;

		for i in 0..CAPACITY {
			assert_eq!(fixture.len, CAPACITY - i);
			assert_eq!(fixture.head, 0);
			fixture.pop_index().unwrap();
		}

		assert_eq!(fixture.len, 0);
		assert_eq!(fixture.head, 0);

		assert_matches!(fixture.pop_index(), Err(crate::Error::Empty));
	}

	#[test]
	fn real_to_virtual() {
		let mut fixture = fixture();

		for (h, l) in (0..CAPACITY).flat_map(|h| (0..=CAPACITY).map(move |i| (h, i))) {
			fixture.head = h;
			fixture.len = l;

			if l == 0 {
				assert_matches!(
					fixture.real_to_virtual(0),
					Err(Error::IndexOutOfRange { index: _, len: _ })
				);
			}

			for i in 0..l {
				if fixture.real_to_virtual(i).unwrap() != expected_real_to_virtual::<CAPACITY>(i, h)
				{
					assert_eq!(
						fixture.real_to_virtual(i).unwrap(),
						expected_real_to_virtual::<CAPACITY>(i, h)
					)
				}
			}
		}
	}

	#[test]
	fn virtual_to_real() {
		let mut fixture = fixture();

		for h in 0..CAPACITY {
			for l in 0..=CAPACITY {
				fixture.head = h;
				fixture.len = l;

				if l == 0 {
					assert_matches!(
						fixture.virtual_to_real(0),
						Err(Error::IndexOutOfRange { index: _, len: _ })
					);
				} else {
					for i in 0..l {
						assert_eq!(
							fixture.virtual_to_real(i).unwrap(),
							expected_virtual_to_real::<CAPACITY>(i, h),
							"h:{} l:{} i:{}",
							h,
							l,
							i,
						);
					}
				}
			}
		}
	}

	#[test]
	fn capacity() {
		let mut fixture = fixture();

		for (h, l) in (0..CAPACITY).flat_map(|h| (0..=CAPACITY).map(move |i| (h, i))) {
			fixture.head = h;
			fixture.len = l;
			assert_eq!(fixture.capacity(), CAPACITY);
		}
	}

	#[test]
	fn len() {
		let mut fixture = fixture();

		for (h, l) in (0..CAPACITY).flat_map(|h| (0..=CAPACITY).map(move |i| (h, i))) {
			fixture.head = h;
			fixture.len = l;
			assert_eq!(fixture.len(), l);
		}
	}
}
