use std::ops::Index;

pub trait CircularBuffer<T>: Index<usize, Output = T> {
	type Iter<'a>: Iterator<Item = &'a T>
	where
		T: 'a,
		Self: 'a;
	fn capacity(&self) -> usize;
	fn push(&mut self, item: T);
	fn iter(&self) -> Self::Iter<'_>;
	fn len(&self) -> usize;
}
