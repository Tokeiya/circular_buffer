#![cfg(test)]
use super::index_coordinator::FixedIndexCoordinator;
use crate::error::*;
use std::assert_matches;

pub trait IndexCoordinatorTestExtensions<const N: usize>: FixedIndexCoordinator<N> {
	fn mut_len(&mut self) -> &mut usize;
	fn mut_head(&mut self) -> &mut usize;

	fn fixture() -> Self;
}

fn expected_resolve_index<const N: usize>(index: usize, head: usize) -> usize {
	(index + head) % N
}

pub(super) fn default<const N: usize, T: IndexCoordinatorTestExtensions<N>>() {
	let mut fixture = T::default();
	assert_eq!(*fixture.mut_len(), 0);
	assert_eq!(*fixture.mut_head(), 0);
}

//noinspection DuplicatedCode
pub(super) fn head_index<const N: usize, T: IndexCoordinatorTestExtensions<N>>() {
	let mut fixture = T::fixture();
	assert_matches!(fixture.head_index(), Err(Error::Empty));

	for (h, l) in (0..N).flat_map(|h| (0..N).map(move |l| (h, l))) {
		*fixture.mut_head() = h;
		*fixture.mut_len() = l;

		if l == 0 {
			assert_matches!(fixture.head_index(), Err(Error::Empty));
		} else {
			assert_eq!(fixture.head_index().unwrap(), h);
		}
	}
}

pub(super) fn tail_index<const N: usize, T: IndexCoordinatorTestExtensions<N>>() {
	let mut fixture = T::fixture();
	assert_matches!(fixture.tail_index(), Err(Error::Empty));

	for (h, l) in (0..N).flat_map(|h| (0..N).map(move |l| (h, l))) {
		*fixture.mut_head() = h;
		*fixture.mut_len() = l;

		if l == 0 {
			assert_matches!(fixture.tail_index(), Err(Error::Empty));
		} else {
			assert_eq!(
				fixture.tail_index().unwrap(),
				expected_resolve_index::<N>(l - 1, h)
			);
		}
	}
}

pub(super) fn enqueue_index<const N: usize, T: IndexCoordinatorTestExtensions<N>>() {
	let mut fixture = T::fixture();

	for i in 0..N {
		assert_eq!(*fixture.mut_len(), i);
		assert_eq!(*fixture.mut_head(), 0);
		fixture.enqueue_index();
	}

	for i in 0..N {
		assert_eq!(*fixture.mut_len(), N);
		assert_eq!(*fixture.mut_head(), i);
		fixture.enqueue_index();
	}
}

//noinspection DuplicatedCode
pub(super) fn dequeue_index<const N: usize, T: IndexCoordinatorTestExtensions<N>>() {
	let mut fixture = T::fixture();
	assert_matches!(fixture.dequeue_index(), Err(Error::Empty));

	*fixture.mut_len() = N;

	for i in 0..N {
		assert_eq!(*fixture.mut_head(), i);
		assert_eq!(*fixture.mut_len(), N - i);
		fixture.dequeue_index().unwrap();
	}

	assert_eq!(*fixture.mut_head(), 0);
	assert_eq!(*fixture.mut_len(), 0);

	assert_matches!(fixture.dequeue_index(), Err(Error::Empty));
}

//noinspection DuplicatedCode
pub(super) fn pop_index<const N: usize, T: IndexCoordinatorTestExtensions<N>>() {
	let mut fixture = T::fixture();
	assert_matches!(fixture.pop_index(), Err(Error::Empty));

	*fixture.mut_len() = N;

	for i in 0..N {
		assert_eq!(*fixture.mut_len(), N - i);
		assert_eq!(*fixture.mut_head(), 0);
		fixture.pop_index().unwrap();
	}

	assert_eq!(*fixture.mut_len(), 0);
	assert_eq!(*fixture.mut_head(), 0);

	assert_matches!(fixture.pop_index(), Err(Error::Empty));
}

pub(super) fn resolve_index<const N: usize, T: IndexCoordinatorTestExtensions<N>>() {
	let mut fixture = T::fixture();

	for h in 0..N {
		for l in 0..=N {
			*fixture.mut_head() = h;
			*fixture.mut_len() = l;

			if l == 0 {
				assert_matches!(
					fixture.resolve_index(0),
					Err(Error::IndexOutOfRange { index: _, len: _ })
				);
			} else {
				for i in 0..l {
					assert_eq!(
						fixture.resolve_index(i).unwrap(),
						expected_resolve_index::<N>(i, h),
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

//noinspection DuplicatedCode
pub(super) fn capacity<const N: usize, T: IndexCoordinatorTestExtensions<N>>() {
	let mut fixture = T::fixture();

	for (h, l) in (0..N).flat_map(|h| (0..=N).map(move |i| (h, i))) {
		*fixture.mut_head() = h;
		*fixture.mut_len() = l;
		assert_eq!(fixture.capacity(), N);
	}
}

//noinspection DuplicatedCode
pub(super) fn len<const N: usize, T: IndexCoordinatorTestExtensions<N>>() {
	let mut fixture = T::fixture();

	for (h, l) in (0..N).flat_map(|h| (0..=N).map(move |i| (h, i))) {
		*fixture.mut_head() = h;
		*fixture.mut_len() = l;
		assert_eq!(fixture.len(), l);
	}
}

pub(super) fn is_empty<const N: usize, T: IndexCoordinatorTestExtensions<N>>() {
	let mut fixture = T::fixture();
	assert_eq!(*fixture.mut_len(), 0);
	assert!(fixture.is_empty());

	for l in 1..=N {
		*fixture.mut_len() = l;
		assert!(!fixture.is_empty());
	}
}

pub(super) fn is_full<const N: usize, T: IndexCoordinatorTestExtensions<N>>() {
	let mut fixture = T::fixture();

	for l in 0..N {
		*fixture.mut_len() = l;
		assert!(!fixture.is_full());
	}

	*fixture.mut_len() = N;
	assert!(fixture.is_full());
}

pub(super) fn clone<const N: usize, T: IndexCoordinatorTestExtensions<N>>() {
	let mut fixture = T::fixture();
	*fixture.mut_len() = 10;
	*fixture.mut_head() = 5;

	let mut actual = fixture.clone();
	assert_eq!(*actual.mut_head(), 5);
	assert_eq!(*actual.mut_len(), 10);
	assert_eq!(actual.capacity(), N);
}
