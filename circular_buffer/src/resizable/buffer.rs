use super::IndexCoordinator;
use super::iter::Iter;
use super::iter_mut::IterMut;
use crate::CircularBuffer;
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};
#[cfg(test)]
use std::{cell::Cell, rc::Rc};

pub struct Buffer<T, C: IndexCoordinator> {
	#[cfg(test)]
	probe: Option<Rc<Cell<usize>>>,
	pub(super) storage: Vec<MaybeUninit<T>>,
	pub(super) coordinator: C,
}

impl<T, C: IndexCoordinator> Buffer<T, C> {
	pub fn new(coordinator: C) -> Self {
		todo!()
	}
}

impl<T, C: IndexCoordinator> Index<usize> for Buffer<T, C> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		todo!()
	}
}

impl<T, C: IndexCoordinator> IndexMut<usize> for Buffer<T, C> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		todo!()
	}
}

impl<T, C: IndexCoordinator> CircularBuffer<T> for Buffer<T, C> {
	type Iter<'a>
		= Iter<'a, T, C>
	where
		T: 'a,
		Self: 'a;
	type MutIter<'a>
		= IterMut<'a, T, C>
	where
		T: 'a,
		Self: 'a;

	fn capacity(&self) -> usize {
		todo!()
	}

	fn enqueue(&mut self, item: T) {
		todo!()
	}

	fn dequeue(&mut self) -> Option<T> {
		todo!()
	}

	fn iter(&self) -> Self::Iter<'_> {
		todo!()
	}

	fn iter_mut(&mut self) -> Self::MutIter<'_> {
		todo!()
	}

	fn len(&self) -> usize {
		todo!()
	}
}
