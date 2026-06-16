use std::iter::{ExactSizeIterator, FusedIterator};
use std::ops::{Index, IndexMut};

/// A fixed-capacity circular buffer.
///
/// This trait provides an abstraction over a FIFO-style circular buffer whose
/// elements can be accessed in logical order.
///
/// Indexing through [`Index`] and [`IndexMut`] uses logical indices, not the
/// buffer's internal physical indices. In other words, `buffer[0]` refers to
/// the oldest element currently stored in the buffer.
///
/// # Indexing
///
/// Valid indices are in the range `0..self.len()`.
///
/// Accessing an out-of-bounds index may panic, following the usual contract of
/// [`Index`] and [`IndexMut`].
///
/// # Capacity
///
/// The capacity is expected to remain constant after the buffer is created.
/// The number of stored elements is always in the range `0..=capacity()`.
///
/// # Overwrite behavior
///
/// When [`enqueue`](Self::enqueue) is called while the buffer is full, the
/// oldest element is discarded and the new element is appended to the back of
/// the buffer.
pub trait CircularBuffer<T>: Index<usize, Output = T> + IndexMut<usize> {
	/// The immutable iterator type.
	///
	/// The iterator visits elements from the logical front to the logical back
	/// of the buffer. This corresponds to the order of `self[0]`, `self[1]`,
	/// and so on.
	type Iter<'a>: Iterator<Item = &'a T> + DoubleEndedIterator + FusedIterator + ExactSizeIterator
	where
		T: 'a,
		Self: 'a;

	/// The mutable iterator type.
	///
	/// The iterator visits elements from the logical front to the logical back
	/// of the buffer. This corresponds to the order of `self[0]`, `self[1]`,
	/// and so on.
	type MutIter<'a>: Iterator<Item = &'a mut T>
		+ DoubleEndedIterator
		+ FusedIterator
		+ ExactSizeIterator
	where
		T: 'a,
		Self: 'a;

	/// Returns the maximum number of elements the buffer can hold.
	///
	/// This value is normally fixed for the lifetime of the buffer.
	fn capacity(&self) -> usize;

	/// Appends an element to the back of the buffer.
	///
	/// If the buffer has spare capacity, the element is inserted after the
	/// current last element.
	///
	/// If the buffer is full, the oldest element is discarded and the new
	/// element is appended to the back of the buffer.
	///
	/// After this operation, [`len`](Self::len) never exceeds
	/// [`capacity`](Self::capacity).
	fn enqueue(&mut self, item: T);

	/// Removes and returns the oldest element from the buffer.
	///
	/// If the buffer is empty, this returns `None`.
	fn dequeue(&mut self) -> Option<T>;

	/// Returns an immutable iterator over the elements.
	///
	/// Elements are yielded from the logical front to the logical back.
	fn iter(&self) -> Self::Iter<'_>;

	/// Returns a mutable iterator over the elements.
	///
	/// Elements are yielded from the logical front to the logical back.
	fn iter_mut(&mut self) -> Self::MutIter<'_>;

	/// Returns the number of elements currently stored in the buffer.
	///
	/// The returned value is always in the range `0..=self.capacity()`.
	fn len(&self) -> usize;

	/// Returns `true` if the buffer contains no elements.
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Removes all elements from the buffer.
	///
	/// After this operation, [`len`](Self::len) returns `0`.
	/// The capacity of the buffer is unchanged.
	fn clear(&mut self);
}
#[cfg(test)]
mod tests {
	use super::*;
	
	#[derive(Default)]
	struct Dummy(pub usize);

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
			self.0
		}

		fn clear(&mut self) {
			unimplemented!()
		}
	}

	#[test]
	fn is_empty() {
		let mut fixture = Dummy::default();
		assert_eq!(fixture.len(), 0);
		assert!(fixture.is_empty());

		for i in 1..100 {
			fixture.0 = i;
			assert_eq!(fixture.len(), i);
			assert!(!fixture.is_empty());
		}
	}
}
