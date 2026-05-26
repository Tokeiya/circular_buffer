use super::iter::Iter;
use crate::circular_buffer::CircularBuffer;
use std::ops::{Index, IndexMut};

pub struct Buffer<T, const N: usize> {
	storage: [T; N],
	head: usize,
	len: usize,
}

impl<T, const N: usize> Default for Buffer<T, N> {
	fn default() -> Self {
		todo!()
	}
}

impl<T, const N: usize> Index<usize> for Buffer<T, N> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		todo!()
	}
}

impl<T, const N: usize> IndexMut<usize> for Buffer<T, N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		todo!()
	}
}

impl<T, const N: usize> CircularBuffer<T> for Buffer<T, N> {
	type Iter<'a>
		= Iter<'a, T, N>
	where
		T: 'a,
		Self: 'a;

	fn capacity(&self) -> usize {
		todo!()
	}

	fn push(&mut self, item: T) {
		todo!()
	}

	fn pop(&mut self) -> Option<T> {
		todo!()
	}

	fn iter(&self) -> Self::Iter<'_> {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	type Fixture = Buffer<u8, 8>;
}
