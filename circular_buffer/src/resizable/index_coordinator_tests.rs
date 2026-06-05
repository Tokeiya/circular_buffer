#![cfg(test)]
use super::index_coordinator::IndexCoordinator;
use crate::error::*;
use std::assert_matches;

fn expected_real_to_virtual(capacity: usize, index: usize, head: usize) -> usize {
	(index + capacity - head) % capacity
}

fn expected_virtual_to_real(capacity: usize, index: usize, head: usize) -> usize {
	(index + head) % capacity
}

pub(super) trait IndexCoordinatorTestExtension: IndexCoordinator + Sized {
	fn fixture(capacity: usize) -> Self;
	fn mut_capacity(&mut self) -> &mut usize;
	fn mut_head(&mut self) -> &mut usize;
	fn mut_len(&mut self) -> &mut usize;

	//noinspection DuplicatedCode
	fn head_index(capacity: usize) {
		let mut fixture = Self::fixture(capacity);
		for (h, l) in (0..capacity).flat_map(|h| (0..=capacity).map(move |l| (h, l))) {
			*fixture.mut_head() = h;
			*fixture.mut_len() = l;

			if l == 0 {
				assert_matches!(fixture.head_index(), Err(Error::Empty));
			} else {
				assert_eq!(fixture.head_index().unwrap(), h);
			}
		}
	}

	fn tail_index(capacity: usize) {
		let mut fixture = Self::fixture(capacity);

		for (h, l) in (0..capacity).flat_map(|h| (0..=capacity).map(move |l| (h, l))) {
			*fixture.mut_head() = h;
			*fixture.mut_len() = l;

			if l == 0 {
				assert_matches!(fixture.tail_index(), Err(Error::Empty));
			} else {
				assert_eq!(
					fixture.tail_index().unwrap(),
					expected_virtual_to_real(capacity, l - 1, h)
				);
			}
		}
	}

	fn enqueue_index(capacity: usize) {
		let mut fixture = Self::fixture(capacity);

		for i in 0..capacity {
			assert_eq!(*fixture.mut_len(), i);
			assert_eq!(*fixture.mut_head(), 0);
			fixture.enqueue_index();
		}

		for i in 0..capacity {
			assert_eq!(*fixture.mut_len(), capacity);
			assert_eq!(*fixture.mut_head(), i);
			fixture.enqueue_index();
		}
	}

	//noinspection DuplicatedCode
	fn dequeue_index(capacity: usize) {
		let mut fixture = Self::fixture(capacity);
		assert_matches!(fixture.dequeue_index(), Err(Error::Empty));

		for i in 0..capacity {
			assert_eq!(*fixture.mut_head(), i);
			assert_eq!(*fixture.mut_len(), capacity - i);
			fixture.dequeue_index().unwrap();
		}

		assert_eq!(*fixture.mut_head(), 0);
		assert_eq!(*fixture.mut_len(), 0);

		assert_matches!(fixture.dequeue_index(), Err(Error::Empty));
	}

	//noinspection DuplicatedCode
	fn pop_index(capacity: usize) {
		let mut fixture = Self::fixture(capacity);

		assert_matches!(fixture.pop_index(), Err(Error::Empty));

		*fixture.mut_len() = capacity;

		for i in 0..capacity {
			assert_eq!(*fixture.mut_len(), capacity - i);
			assert_eq!(*fixture.mut_head(), 0);
			fixture.pop_index().unwrap();
		}

		assert_eq!(*fixture.mut_len(), 0);
		assert_eq!(*fixture.mut_head(), 0);

		assert_matches!(fixture.pop_index(), Err(Error::Empty));
	}

	fn real_to_virtual(capacity: usize) {
		let mut fixture = Self::fixture(capacity);

		for (h, l) in (0..capacity).flat_map(|h| (0..=capacity).map(move |i| (h, i))) {
			*fixture.mut_head() = h;
			*fixture.mut_len() = l;

			if l == 0 {
				assert_matches!(
					fixture.real_to_virtual(0),
					Err(Error::IndexOutOfRange { index: _, len: _ })
				);
			}

			for i in 0..l {
				if fixture.real_to_virtual(i).unwrap() != expected_real_to_virtual(capacity, i, h) {
					assert_eq!(
						fixture.real_to_virtual(i).unwrap(),
						expected_real_to_virtual(capacity, i, h)
					)
				}
			}
		}
	}

	fn virtual_to_real(capacity: usize) {
		let mut fixture = Self::fixture(capacity);

		for h in 0..capacity {
			for l in 0..=capacity {
				*fixture.mut_head() = h;
				*fixture.mut_len() = l;

				if l == 0 {
					assert_matches!(
						fixture.virtual_to_real(0),
						Err(Error::IndexOutOfRange { index: _, len: _ })
					);
				} else {
					for i in 0..l {
						assert_eq!(
							fixture.virtual_to_real(i).unwrap(),
							expected_virtual_to_real(capacity, i, h),
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
	fn capacity(capacity: usize) {
		let mut fixture = Self::fixture(capacity);

		for (h, l) in (0..capacity).flat_map(|h| (0..=capacity).map(move |i| (h, i))) {
			*fixture.mut_head() = h;
			*fixture.mut_len() = l;
			assert_eq!(fixture.capacity(), capacity);
		}
	}

	//noinspection DuplicatedCode
	fn len(capacity: usize) {
		let mut fixture = Self::fixture(capacity);

		for (h, l) in (0..capacity).flat_map(|h| (0..=capacity).map(move |i| (h, i))) {
			*fixture.mut_head() = h;
			*fixture.mut_len() = l;
			assert_eq!(fixture.len(), l);
		}
	}

	fn is_empty(capacity: usize) {
		let mut fixture = Self::fixture(capacity);
		assert_eq!(*fixture.mut_len(), 0);
		assert!(fixture.is_empty());

		for l in 1..=capacity {
			*fixture.mut_len() = l;
			assert!(!fixture.is_empty());
		}
	}

	fn is_full(capacity: usize) {
		let mut fixture = Self::fixture(capacity);

		for l in 0..capacity {
			*fixture.mut_len() = l;
			assert!(!fixture.is_full());
		}

		*fixture.mut_len() = capacity;
		assert!(fixture.is_full());
	}

	fn clone(capacity: usize) {
		let mut fixture = Self::fixture(capacity);
		*fixture.mut_len() = 10;
		*fixture.mut_head() = 5;

		let mut actual = fixture.clone();
		assert_eq!(*actual.mut_head(), 5);
		assert_eq!(*actual.mut_len(), 10);
		assert_eq!(*actual.mut_capacity(), capacity);
	}
}
