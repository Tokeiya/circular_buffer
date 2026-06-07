use super::IndexCoordinator;
use super::buffer::Buffer;
use std::iter::{DoubleEndedIterator, ExactSizeIterator, FusedIterator, Iterator};
pub struct Iter<'a, T, C: IndexCoordinator> {
	backend: &'a Buffer<T, C>,
	coordinator: C,
}

impl<'a, T, C: IndexCoordinator> Iter<'a, T, C> {
	pub(super) fn new(buffer: &'a Buffer<T, C>) -> Self {
		todo!()
	}
}

impl<'a, T, C: IndexCoordinator> Iterator for Iter<'a, T, C> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		todo!()
	}
}

impl<'a, T, C: IndexCoordinator> FusedIterator for Iter<'a, T, C> {}

impl<'a, T, C: IndexCoordinator> ExactSizeIterator for Iter<'a, T, C> {}

impl<'a, T, C: IndexCoordinator> DoubleEndedIterator for Iter<'a, T, C> {
	fn next_back(&mut self) -> Option<Self::Item> {
		todo!()
	}
}
