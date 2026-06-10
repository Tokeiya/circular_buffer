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
