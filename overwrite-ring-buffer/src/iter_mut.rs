use super::CircularBuffer;
use super::IndexCoordinator;
use std::iter::FusedIterator;
use std::marker::PhantomData;

/// A mutable iterator over the elements of a circular buffer.
///
/// This iterator yields unique mutable references to the initialized elements
/// currently represented by the coordinator.
///
/// Elements are yielded in logical order, from the front of the buffer to the
/// back of the buffer.
///
/// The iterator owns a snapshot of the index coordinator. Advancing the
/// iterator updates only this local coordinator state and does not modify the
/// original buffer's coordinator.
///
/// # Safety invariants
///
/// This iterator is created from a mutable borrow of the original buffer.
/// The raw pointer stored in this iterator must point to the beginning of the
/// buffer's backing storage and must remain valid for the lifetime `'a`.
///
/// The coordinator must only yield physical indices that refer to initialized
/// elements, and each element must be yielded at most once. This is required to
/// uphold the uniqueness guarantee of `&'a mut T`.
pub struct IterMut<'a, T, C> {
	// Pointer to the first slot of the backing storage.
	head_ptr: *mut std::mem::MaybeUninit<T>,

	// A local snapshot of the buffer's logical index state.
	coordinator: C,

	// Ties this iterator to the mutable borrow of the original buffer.
	_phantom: PhantomData<&'a mut T>,
}

impl<'a, T, C> IterMut<'a, T, C> {
	/// Creates a new mutable iterator.
	///
	/// The mutable buffer reference is used to tie the iterator lifetime to the
	/// exclusive borrow of the original buffer.
	///
	/// The caller must provide a pointer to the beginning of the backing
	/// storage and a coordinator that describes the initialized elements.
	pub(super) fn new<B: CircularBuffer<T>>(
		_: &'a mut B,
		head_pointer: *mut std::mem::MaybeUninit<T>,
		coordinator: C,
	) -> Self {
		Self {
			head_ptr: head_pointer,
			coordinator,
			_phantom: PhantomData,
		}
	}
}

impl<'a, T: 'a, C: IndexCoordinator> Iterator for IterMut<'a, T, C> {
	type Item = &'a mut T;

	/// Returns the next element from the logical front of the buffer.
	fn next(&mut self) -> Option<Self::Item> {
		if self.coordinator.is_empty() {
			None
		} else {
			let ret = unsafe {
				// SAFETY:
				// The coordinator is expected to point only to initialized
				// elements. `head_index` returns the physical index of the
				// current logical front element.
				//
				// After this reference is created, the coordinator is advanced
				// so that the same element will not be yielded again.
				let uninit_ref = &mut *self.head_ptr.add(self.coordinator.head_index().unwrap());
				Some(uninit_ref.assume_init_mut())
			};

			self.coordinator.dequeue_index().unwrap();
			ret
		}
	}

	/// Returns the exact number of remaining elements.
	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.coordinator.len(), Some(self.coordinator.len()))
	}
}

impl<'a, T: 'a, C: IndexCoordinator> DoubleEndedIterator for IterMut<'a, T, C> {
	/// Returns the next element from the logical back of the buffer.
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.coordinator.is_empty() {
			None
		} else {
			let ret = unsafe {
				let index = self.coordinator.tail_index().unwrap();

				// SAFETY:
				// The coordinator is expected to point only to initialized
				// elements. `tail_index` returns the physical index of the
				// current logical back element.
				//
				// After this reference is created, the coordinator is moved
				// backward so that the same element will not be yielded again.
				let uninit_ref = &mut *self.head_ptr.add(index);
				Some(uninit_ref.assume_init_mut())
			};

			self.coordinator.pop_index().unwrap();
			ret
		}
	}
}

impl<'a, T: 'a, C: IndexCoordinator> FusedIterator for IterMut<'a, T, C> {}

impl<'a, T: 'a, C: IndexCoordinator> ExactSizeIterator for IterMut<'a, T, C> {
	/// Returns the number of remaining elements.
	fn len(&self) -> usize {
		self.coordinator.len()
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	use crate::CircularBuffer;
	use crate::fixed::index_coordinator_test::IndexCoordinatorTestExtensions;
	use crate::iter::Iter;
	use std::mem::MaybeUninit;
	use std::ops::{Index, IndexMut};

	pub struct Dummy;

	fn expected_virtual_to_real(capacity: usize, index: usize, head: usize) -> usize {
		(index + head) % capacity
	}

	fn sample() -> [MaybeUninit<usize>; SIZE] {
		std::array::from_fn(|i| MaybeUninit::new(i))
	}

	#[derive(Debug)]
	struct PairIter {
		pub head: usize,
		pub len: usize,
	}

	fn pair_iter() -> impl Iterator<Item = PairIter> {
		(0..SIZE).flat_map(|h| (0..=SIZE).map(move |l| PairIter { head: h, len: l }))
	}

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

		fn clear(&mut self) {
			unimplemented!()
		}
	}

	const SIZE: usize = 8;
	type Coordinator = crate::fixed::Pow2IndexCoordinator<SIZE>;

	#[test]
	fn new() {
		let mut coordinator = Coordinator::default();
		*coordinator.mut_head() = SIZE / 2;
		*coordinator.mut_len() = SIZE;

		let mut dummy = Dummy;
		let mut scr = sample();

		let mut iter = IterMut::new(&mut dummy, scr.as_mut_ptr(), coordinator);
		assert_eq!(*iter.coordinator.mut_head(), SIZE / 2);
		assert_eq!(*iter.coordinator.mut_len(), SIZE);
		assert_eq!(iter.head_ptr, scr.as_mut_ptr());
	}

	#[test]
	fn index_test() {
		let mut c = Coordinator::default();
		*c.mut_len() = 1;
		*c.mut_head() = 1;

		for i in 0..*c.mut_len() {
			println!(
				"v[{i}]=r[{}]=e[{}]",
				c.resolve_index(i).unwrap(),
				expected_virtual_to_real(SIZE, i, *c.mut_head())
			);
		}
	}

	#[allow(clippy::while_let_on_iterator)]
	#[test]
	fn next() {
		const OFFSET: usize = 1_000;

		for env in pair_iter() {
			let mut coordinator = Coordinator::default();
			*coordinator.mut_head() = env.head;
			*coordinator.mut_len() = env.len;
			let mut dummy = Dummy;

			let mut sample: [MaybeUninit<usize>; SIZE] =
				std::array::from_fn(|_| MaybeUninit::new(500));
			for (i, idx) in (0..env.len).map(|i| (env.head + i) % SIZE).enumerate() {
				sample[idx] = MaybeUninit::new(i);
			}

			let mut fixture = IterMut::new(&mut dummy, sample.as_mut_ptr(), coordinator.clone());
			let mut cnt = 0usize;

			while let Some(elem) = fixture.next() {
				assert_eq!(
					*elem, cnt,
					"act:{:?} exp:{:?} head:{:?} len:{:?}",
					*elem, cnt, env.head, env.len
				);
				cnt += 1;
				*elem += OFFSET;
			}

			assert_eq!(cnt, env.len);
			assert!(fixture.next().is_none());

			let iter = Iter::new(sample.as_slice(), coordinator.clone());

			for (i, act) in iter.enumerate() {
				assert_eq!(i + OFFSET, *act);
			}
		}
	}

	#[test]
	fn size_hint_len() {
		for env in pair_iter() {
			let mut coordinator = Coordinator::default();
			*coordinator.mut_head() = env.head;
			*coordinator.mut_len() = env.len;

			let mut dummy = Dummy;
			let mut sample = sample();

			let mut fixture = IterMut::new(&mut dummy, sample.as_mut_ptr(), coordinator.clone());

			let mut expected = env.len;

			while fixture.next().is_some() {
				expected -= 1;
				assert_eq!(
					fixture.size_hint(),
					(expected, Some(expected)),
					"act:{:?} exp:{:?} head:{} len:{}",
					fixture.size_hint(),
					(expected, Some(expected)),
					env.head,
					env.len
				);
				assert_eq!(fixture.len(), expected);
			}

			assert_eq!(fixture.size_hint(), (0, Some(0)));
			assert_eq!(fixture.len(), 0);

			fixture = IterMut::new(&mut dummy, sample.as_mut_ptr(), coordinator.clone());
			expected = env.len;

			while fixture.next_back().is_some() {
				expected -= 1;
				assert_eq!(fixture.size_hint(), (expected, Some(expected)));
				assert_eq!(fixture.len(), expected);
			}
		}
	}

	#[allow(clippy::needless_range_loop)]
	#[test]
	fn next_back() {
		const OFFSET: usize = 1_000;

		for env in pair_iter() {
			let mut scr = sample();
			let mut coordinator = Coordinator::default();

			*coordinator.mut_len() = env.len;
			*coordinator.mut_head() = env.head;
			let mut dummy = Dummy;
			let mut fixture = IterMut::new(&mut dummy, scr.as_mut_ptr(), coordinator.clone());

			if env.len == 0 {
				assert!(fixture.next_back().is_none());
			} else {
				for idx in (0..env.len).rev() {
					let act = fixture.next_back().unwrap();
					assert_eq!(act, unsafe {
						scr[expected_virtual_to_real(SIZE, idx, env.head)].assume_init_ref()
					});

					*act += OFFSET;
				}

				let iter = Iter::new(scr.as_slice(), coordinator.clone());

				for (i, act) in iter.enumerate() {
					assert_eq!(*act, ((i + env.head) % SIZE) + OFFSET);
				}
			}
		}
	}

	#[test]
	fn complex() {
		let mut scr = sample();
		let mut coordinator = Coordinator::default();
		*coordinator.mut_len() = 8;

		let mut dummy = Dummy;
		let mut fixture = IterMut::new(&mut dummy, scr.as_mut_ptr(), coordinator);

		assert_eq!(fixture.len(), 8);
		assert_eq!(fixture.size_hint(), (8, Some(8)));

		fixture.next().unwrap();
		assert_eq!(fixture.len(), 7);
		assert_eq!(fixture.size_hint(), (7, Some(7)));

		fixture.next_back().unwrap();
		assert_eq!(fixture.len(), 6);
		assert_eq!(fixture.size_hint(), (6, Some(6)));

		fixture.next().unwrap();
		assert_eq!(fixture.len(), 5);
		assert_eq!(fixture.size_hint(), (5, Some(5)));

		fixture.next_back().unwrap();
		assert_eq!(fixture.len(), 4);
		assert_eq!(fixture.size_hint(), (4, Some(4)));

		fixture.next().unwrap();
		assert_eq!(fixture.len(), 3);
		assert_eq!(fixture.size_hint(), (3, Some(3)));

		fixture.next_back().unwrap();
		assert_eq!(fixture.len(), 2);
		assert_eq!(fixture.size_hint(), (2, Some(2)));

		fixture.next().unwrap();
		assert_eq!(fixture.len(), 1);
		assert_eq!(fixture.size_hint(), (1, Some(1)));

		fixture.next_back().unwrap();
		assert_eq!(fixture.len(), 0);
		assert_eq!(fixture.size_hint(), (0, Some(0)));

		assert!(fixture.next().is_none());
		assert!(fixture.next_back().is_none());

		assert_eq!(fixture.len(), 0);
		assert_eq!(fixture.size_hint(), (0, Some(0)));
	}
}
