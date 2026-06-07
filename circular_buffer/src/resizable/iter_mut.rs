use super::IndexCoordinator;
use super::buffer::Buffer;
use std::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator, Iterator};
pub struct IterMut<'a, T, C: IndexCoordinator> {
	backend: *mut std::mem::MaybeUninit<T>,
	coordinator: C,
	_phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T, C: IndexCoordinator> IterMut<'a, T, C> {
	pub(super) fn new(buffer: &'a mut Buffer<T, C>) -> Self {
		todo!()
	}
}

impl<'a, T, C: IndexCoordinator> Iterator for IterMut<'a, T, C> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		todo!()
	}
}

impl<'a, T, C: IndexCoordinator> DoubleEndedIterator for IterMut<'a, T, C> {
	fn next_back(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

impl<'a, T, C: IndexCoordinator> FusedIterator for IterMut<'a, T, C> {}

impl<'a, T, C: IndexCoordinator> ExactSizeIterator for IterMut<'a, T, C> {
	fn len(&self) -> usize {
		todo!()
	}
}
