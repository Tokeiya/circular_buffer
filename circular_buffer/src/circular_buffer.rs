use std::iter::{ExactSizeIterator, FusedIterator};
use std::ops::{Index, IndexMut};

pub trait CircularBuffer<T>: Index<usize, Output = T> + IndexMut<usize> {
	type Iter<'a>: Iterator<Item = &'a T> + DoubleEndedIterator + FusedIterator + ExactSizeIterator
	where
		T: 'a,
		Self: 'a;

	type MutIter<'a>: Iterator<Item = &'a mut T>
		+ DoubleEndedIterator
		+ FusedIterator
		+ ExactSizeIterator
	where
		T: 'a,
		Self: 'a;

	fn capacity(&self) -> usize;
	fn enqueue(&mut self, item: T);
	fn dequeue(&mut self) -> Option<T>;
	fn iter(&self) -> Self::Iter<'_>;
	fn iter_mut(&mut self) -> Self::MutIter<'_>;
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
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
			todo!()
		}

		fn enqueue(&mut self, _: usize) {
			todo!()
		}

		fn dequeue(&mut self) -> Option<usize> {
			todo!()
		}

		fn iter(&self) -> Self::Iter<'_> {
			todo!()
		}

		fn iter_mut(&mut self) -> Self::MutIter<'_> {
			todo!()
		}

		fn len(&self) -> usize {
			self.0
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
