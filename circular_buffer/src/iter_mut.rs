use super::CircularBuffer;
use super::IndexCoordinator;
use std::iter::FusedIterator;
pub struct IterMut<'a, T, C> {
	head_ptr: *mut std::mem::MaybeUninit<T>,
	coordinator: C,
	_phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T, C> IterMut<'a, T, C> {
	pub(super) fn new<B: CircularBuffer<T>>(
		_: &'a mut B,
		head_pointer: *mut std::mem::MaybeUninit<T>,
		coordinator: C,
	) -> Self {
		todo!()
	}
}

impl<'a, T: 'a, C: IndexCoordinator> Iterator for IterMut<'a, T, C> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		todo!()
	}
}

impl<'a, T: 'a, C: IndexCoordinator> DoubleEndedIterator for IterMut<'a, T, C> {
	fn next_back(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

impl<'a, T: 'a, C: IndexCoordinator> FusedIterator for IterMut<'a, T, C> {}

impl<'a, T: 'a, C: IndexCoordinator> ExactSizeIterator for IterMut<'a, T, C> {
	fn len(&self) -> usize {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CircularBuffer;
	use std::ops::{Index, IndexMut};

	pub struct Dummy;

	impl Index<usize> for Dummy {
		type Output = usize;

		fn index(&self, _: usize) -> &Self::Output {
			unimplemented!()
		}
	}

	impl IndexMut<usize> for Dummy {
		fn index_mut(&mut self, _: usize) -> &mut Self::Output {
			unimplemented!()
		}
	}

	impl CircularBuffer<usize> for Dummy {
		type Iter<'a> = std::iter::Empty<&'a usize>;
		type MutIter<'a> = std::iter::Empty<&'a mut usize>;

		fn capacity(&self) -> usize {
			unimplemented!()
		}

		fn enqueue(&mut self, _: usize) {
			unimplemented!()
		}

		fn dequeue(&mut self) -> Option<usize> {
			unimplemented!()
		}

		fn iter(&self) -> Self::Iter<'_> {
			unimplemented!()
		}

		fn iter_mut(&mut self) -> Self::MutIter<'_> {
			unimplemented!()
		}

		fn len(&self) -> usize {
			unimplemented!()
		}
	}

	const SIZE: usize = 8;
	type Coordinator = crate::fixed::Pow2IndexCoordinator<SIZE>;

	#[test]
	fn new() {
		todo!();
	}

	#[test]
	fn next() {
		todo!();
	}

	#[test]
	fn size_hint_len() {
		todo!();
	}

	#[test]
	fn next_back() {
		todo!();
	}
}
