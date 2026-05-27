use super::buffer::Buffer;
use crate::circular_buffer::CircularBuffer;

pub struct Iter<'a, T, const N: usize> {
	backend: &'a Buffer<T, N>,
	index: usize,
	len: usize,
}

impl<'a, T, const N: usize> Iter<'a, T, N> {
	pub(super) fn new(item: &'a Buffer<T, N>) -> Self {
		Self {
			backend: item,
			index: 0,
			len: item.len(),
		}
	}
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		todo!("not implemented");
	}
}

impl<'a, T, const N: usize> ExactSizeIterator for Iter<'a, T, N> {
	fn len(&self) -> usize {
		todo!()
	}
}

impl<'a, T, const N: usize> DoubleEndedIterator for Iter<'a, T, N> {
	fn next_back(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::fixed::buffer::Buffer;

	const SIZE: usize = 8;
	type Fixture = Buffer<usize, SIZE>;

	fn fixture() -> Fixture {
		let mut fixture = Buffer::default();

		for i in 0..SIZE {
			fixture.push(i)
		}

		fixture
	}

	#[test]
	fn next() {
		todo!();
	}

	#[test]
	fn len() {
		todo!();
	}

	#[test]
	fn next_back() {
		todo!();
	}

	#[test]
	fn complex() {
		todo!();
	}
}
