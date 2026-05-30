use std::ops::{Index, IndexMut};

pub trait CircularBuffer<T>: Index<usize, Output = T> + IndexMut<usize> {
	type Iter<'a>: Iterator<Item = &'a T>
	where
		T: 'a,
		Self: 'a;

	type MutIter<'a>: Iterator<Item = &'a mut T>
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

// #[cfg(test)]
// mod tests {
//
// 	#[test]
// 	fn test_is_empty() {
// 		todo!("not implemented");
// 	}
// }
