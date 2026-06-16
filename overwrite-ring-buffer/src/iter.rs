pub use super::IndexCoordinator;
use std::iter::FusedIterator;
use std::mem::MaybeUninit;

/// An immutable iterator over the elements of a circular buffer.
///
/// This iterator yields shared references to the initialized elements currently
/// represented by the coordinator.
///
/// Elements are yielded in logical order, from the front of the buffer to the
/// back of the buffer.
///
/// The iterator owns a snapshot of the index coordinator. Advancing the
/// iterator updates only this local coordinator state and does not modify the
/// original buffer.
pub struct Iter<'a, T, C: IndexCoordinator> {
	buff: &'a [MaybeUninit<T>],
	coordinator: C,
}

impl<'a, T, C: IndexCoordinator> Iter<'a, T, C> {
	/// Creates a new immutable iterator.
	///
	/// The provided coordinator must describe only initialized elements within
	/// `buff`.
	pub(crate) fn new(buff: &'a [MaybeUninit<T>], coordinator: C) -> Self {
		Self { buff, coordinator }
	}
}

impl<'a, T, C: IndexCoordinator> Iterator for Iter<'a, T, C> {
	type Item = &'a T;

	/// Returns the next element from the logical front of the buffer.
	fn next(&mut self) -> Option<Self::Item> {
		if self.coordinator.len() == 0 {
			None
		} else {
			// SAFETY:
			// The coordinator is expected to point only to initialized elements.
			// `head_index` returns the physical index of the current logical
			// front element, and the returned reference is tied to the lifetime
			// of the backing buffer.
			let item =
				unsafe { self.buff[self.coordinator.head_index().unwrap()].assume_init_ref() };

			self.coordinator.dequeue_index().unwrap();
			Some(item)
		}
	}

	/// Returns the exact number of remaining elements.
	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.coordinator.len();
		(len, Some(len))
	}
}

impl<'a, T, C: IndexCoordinator> ExactSizeIterator for Iter<'a, T, C> {
	/// Returns the number of remaining elements.
	fn len(&self) -> usize {
		self.coordinator.len()
	}
}

impl<'a, T, C: IndexCoordinator> DoubleEndedIterator for Iter<'a, T, C> {
	/// Returns the next element from the logical back of the buffer.
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.coordinator.len() == 0 {
			None
		} else {
			// SAFETY:
			// The coordinator is expected to point only to initialized elements.
			// `tail_index` returns the physical index of the current logical
			// back element, and the returned reference is tied to the lifetime
			// of the backing buffer.
			let item =
				unsafe { self.buff[self.coordinator.tail_index().unwrap()].assume_init_ref() };

			self.coordinator.pop_index().unwrap();
			Some(item)
		}
	}
}

impl<'a, T, C: IndexCoordinator> FusedIterator for Iter<'a, T, C> {}
#[cfg(test)]
mod test {
	use super::*;
	use crate::fixed::index_coordinator_test::IndexCoordinatorTestExtensions;
	use crate::fixed::GeneralIndexCoordinator;
	use std::array::from_fn;
	
	const SIZE: usize = 8;
	const MASK: usize = SIZE - 1;

	type Coordinator = GeneralIndexCoordinator<SIZE>;

	fn gen_sample() -> [MaybeUninit<usize>; SIZE] {
		from_fn(MaybeUninit::new)
	}

	fn expected_virtual_to_real(capacity: usize, index: usize, head: usize) -> usize {
		(index + head) % capacity
	}

	#[test]
	fn new() {
		let scr = gen_sample();
		let mut coordinator = Coordinator::default();
		*coordinator.mut_head() = SIZE / 2;
		*coordinator.mut_len() = SIZE;

		let mut fixture = Iter::new(scr.as_slice(), coordinator.clone());

		assert_eq!(*fixture.coordinator.mut_head(), SIZE / 2);
		assert_eq!(*fixture.coordinator.mut_len(), SIZE);

		for i in 0..SIZE {
			let act = *unsafe { fixture.buff[i].assume_init_ref() };
			assert_eq!(act, i);
		}
	}

	#[test]
	fn next() {
		for len in 0..SIZE {
			for head in 0..SIZE {
				let mut scr = gen_sample();
				let offset = SIZE - head;

				let mut coordinator = Coordinator::default();
				*coordinator.mut_head() = head;
				*coordinator.mut_len() = len;

				for e in scr.iter_mut() {
					*unsafe { e.assume_init_mut() } += offset;
					*unsafe { e.assume_init_mut() } &= MASK;
				}

				for (e, &a) in Iter::new(scr.as_slice(), coordinator.clone()).enumerate() {
					assert_eq!(a, e)
				}
			}
		}
	}

	#[test]
	fn len_size_hint() {
		let scr = gen_sample();
		let mut coordinator = Coordinator::default();

		for (l, h) in (0..=SIZE).flat_map(|l| (0..SIZE).map(move |h| (l, h))) {
			*coordinator.mut_head() = h;
			*coordinator.mut_len() = l;

			let mut iter = Iter::new(scr.as_slice(), coordinator.clone());

			for i in 0..l {
				assert_eq!(iter.len(), l - i);
				assert_eq!(iter.size_hint(), (l - i, Some(l - i)));
				iter.next().unwrap();
			}

			assert_eq!(iter.len(), 0);
			assert_eq!(iter.size_hint(), (0, Some(0)));
		}
	}

	#[test]
	fn next_back() {
		let scr = gen_sample();
		let mut coordinator = Coordinator::default();

		for (l, h) in (0..=SIZE).flat_map(|l| (0..SIZE).map(move |h| (l, h))) {
			*coordinator.mut_head() = h;
			*coordinator.mut_len() = l;

			let mut iter = Iter::new(scr.as_slice(), coordinator.clone());

			if l == 0 {
				assert_eq!(iter.next_back(), None);
			} else {
				let mut idx = l - 1;

				while let Some(a) = iter.next_back() {
					assert_eq!(a, unsafe {
						scr[expected_virtual_to_real(SIZE, idx, h)].assume_init_ref()
					});
					idx = idx.saturating_sub(1);
				}
			}
		}
	}

	#[test]
	fn complex() {
		let mut scr = gen_sample();
		let mut coordinator = Coordinator::default();

		*coordinator.mut_head() = SIZE / 2;
		*coordinator.mut_len() = SIZE;

		for i in 0..SIZE {
			scr[expected_virtual_to_real(SIZE, i, SIZE / 2)].write(i);
		}

		let mut iter = Iter::new(scr.as_slice(), coordinator.clone());

		assert_eq!(iter.next(), Some(&0));
		assert_eq!(iter.next_back(), Some(&7));
		assert_eq!(iter.next(), Some(&1));
		assert_eq!(iter.next_back(), Some(&6));
		assert_eq!(iter.next(), Some(&2));
		assert_eq!(iter.next_back(), Some(&5));
		assert_eq!(iter.next(), Some(&3));
		assert_eq!(iter.next_back(), Some(&4));
		assert_eq!(iter.next(), None);
		assert_eq!(iter.next_back(), None);
		assert_eq!(iter.next(), None);
		assert_eq!(iter.next_back(), None);
	}
}
