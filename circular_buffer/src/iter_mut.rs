use super::{CircularBuffer, IndexCoordinator};
use std::iter::FusedIterator;

pub struct IterMut<'a, T, C: IndexCoordinator> {
	head_ptr: *mut std::mem::MaybeUninit<T>,
	coordinator: C,
	_phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, T, C: IndexCoordinator> IterMut<'a, T, C> {
	pub(crate) fn new(
		_: &'a impl CircularBuffer<T>,
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

impl<'a, T: 'a, C: IndexCoordinator> ExactSizeIterator for IterMut<'a, T, C> {
	fn len(&self) -> usize {
		todo!()
	}
}

impl<'a, T: 'a, C: IndexCoordinator> DoubleEndedIterator for IterMut<'a, T, C> {
	fn next_back(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::fixed::GeneralIndexCoordinator;
	use crate::fixed::index_coordinator_test::IndexCoordinatorTestExtensions;
	use crate::iter::Iter;
	use std::array::from_fn;
	use std::io::Empty;
	use std::ops::{Index, IndexMut};

	struct Dummy;

	impl Index<usize> for Dummy {
		type Output = usize;

		fn index(&self, _: usize) -> &Self::Output {
			todo!()
		}
	}

	impl IndexMut<usize> for Dummy {
		fn index_mut(&mut self, _: usize) -> &mut Self::Output {
			todo!()
		}
	}
	impl CircularBuffer<usize> for Dummy {
		type Iter<'a>
			= std::iter::Empty<&'a usize>
		where
			Self: 'a;

		type MutIter<'a>
			= std::iter::Empty<&'a mut usize>
		where
			Self: 'a;

		fn capacity(&self) -> usize {
			todo!()
		}

		fn enqueue(&mut self, item: usize) {
			todo!()
		}

		fn dequeue(&mut self) -> Option<usize> {
			todo!()
		}

		fn iter(&self) -> std::iter::Empty<&usize> {
			todo!()
		}

		fn iter_mut(&mut self) -> std::iter::Empty<&mut usize> {
			todo!()
		}

		fn len(&self) -> usize {
			todo!()
		}
	}

	const SIZE: usize = 8;
	type Fixture<'a> = IterMut<'a, i64, GeneralIndexCoordinator<SIZE>>;
	type Coordinator = GeneralIndexCoordinator<SIZE>;

	fn gen_sample() -> [usize; SIZE] {
		from_fn(|i| i)
	}

	fn expected_real_to_virtual(capacity: usize, index: usize, head: usize) -> usize {
		(index + capacity - head) % capacity
	}

	fn expected_virtual_to_real(capacity: usize, index: usize, head: usize) -> usize {
		(index + head) % capacity
	}

	// #[test]
	// fn new() {
	// 	let mut c = Coordinator::default();
	// 	*c.mut_head() = SIZE / 2;
	// 	*c.mut_len() = SIZE - 1;
	//
	// 	let scr = gen_sample();
	// 	let dummy = Dummy;
	// 	let fixture = IterMut::new(&dummy, scr.as_mut_ptr(), c.clone());
	// }
}
