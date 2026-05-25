use crate::error::*;

pub struct IndexCoordinator<const N: usize> {
	head: usize,
	len: usize,
}

impl<const N: usize> IndexCoordinator<N> {
	const CHECK: () = assert!(N.count_ones() == 1);
	pub fn new() -> Self {
		todo!()
	}

	pub fn push_index(&mut self) {
		todo!()
	}

	pub fn pop_index(&mut self) -> Result<()> {
		todo!()
	}

	pub fn real_to_virtual(&self, idx: usize) -> Result<usize> {
		todo!()
	}

	pub fn virtual_to_real(&self, idx: usize) -> Result<usize> {
		todo!()
	}

	pub fn capacity(&self) -> usize {
		todo!()
	}

	pub fn len(&self) -> usize {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const BASE: usize = 8;
	fn expected_real_to_virtual<const N: usize>(index: usize, head: usize) -> usize {
		(index + N - head) % N
	}

	fn expected_virtual_to_real<const N: usize>(index: usize, head: usize) -> usize {
		(index + head) % N
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
			fixture.push_index();
			assert_eq!(fixture.capacity(), BASE);
		}

		for _ in 0..BASE {
			fixture.pop_index().unwrap();
			assert_eq!(fixture.capacity(), BASE);
		}

		for _ in 0..BASE {
			fixture.pop_index().unwrap_err();
			assert_eq!(fixture.capacity(), BASE);
		}
	}

	#[test]
	fn push_index() {
		let mut fixture = IndexCoordinator::<BASE>::new();

		for i in 0..BASE {
			fixture.push_index();
			assert_eq!(fixture.len, i + 1);
			assert_eq!(fixture.head, 0);
		}

		for i in 0..BASE {
			fixture.push_index();
			assert_eq!(fixture.len, BASE);
			assert_eq!(fixture.head, i + 1);
		}
	}

	#[test]
	fn pop_index() {
		let mut fixture = IndexCoordinator::<BASE>::new();
		fixture.len = 8;
		fixture.head = 4;

		for i in 0..BASE {
			fixture.pop_index().unwrap();
			assert_eq!(fixture.head, 4);
			assert_eq!(fixture.len, 7 - i);
		}

		for _ in 0..BASE {
			let err = fixture.pop_index().unwrap_err();
			assert!(matches!(err, Error::Empty));
		}
	}

	#[test]
	fn len() {
		let mut fixture = IndexCoordinator::<BASE>::new();

		for i in 0..BASE {
			assert_eq!(fixture.len(), i);
			fixture.push_index();
			assert_eq!(fixture.len(), i + 1);
		}

		for _ in 0..BASE {
			assert_eq!(fixture.len(), BASE);
			fixture.push_index();
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

			fixture.push_index();
		}

		let mut expected = 1usize;

		for _ in 0..100 {
			fixture.push_index();
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

			fixture.push_index();
		}

		let mut expected = 1usize;

		for _ in 0..100 {
			fixture.push_index();
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
}
