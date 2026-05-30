use circular_buffer::CircularBuffer;
use std::collections::{
	VecDeque,
	vec_deque::{Iter, IterMut},
};
use std::ops::{Index, IndexMut};

pub struct Expected<T, const N: usize> {
	storage: VecDeque<T>,
}

impl<T, const N: usize> Default for Expected<T, N> {
	fn default() -> Self {
		Self {
			storage: VecDeque::with_capacity(N),
		}
	}
}

impl<T, const N: usize> IndexMut<usize> for Expected<T, N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.storage[index]
	}
}

impl<T, const N: usize> Index<usize> for Expected<T, N> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self.storage[index]
	}
}

impl<T, const N: usize> CircularBuffer<T> for Expected<T, N> {
	type Iter<'a>
		= Iter<'a, T>
	where
		T: 'a,
		Self: 'a;

	type MutIter<'a>
		= IterMut<'a, T>
	where
		T: 'a,
		Self: 'a;

	fn capacity(&self) -> usize {
		N
	}

	fn enqueue(&mut self, item: T) {
		if self.storage.len() >= self.capacity() {
			drop(self.storage.pop_front().unwrap());
		}
		self.storage.push_back(item);
	}

	fn dequeue(&mut self) -> Option<T> {
		self.storage.pop_front()
	}

	fn iter(&self) -> Self::Iter<'_> {
		self.storage.iter()
	}

	fn iter_mut(&mut self) -> Self::MutIter<'_> {
		self.storage.iter_mut()
	}

	fn len(&self) -> usize {
		self.storage.len()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	const SIZE: usize = 8;
	type Fixture = Expected<usize, SIZE>;
	#[test]
	fn enqueue() {
		let mut fixture = Fixture::default();

		for i in 0..SIZE {
			assert_eq!(fixture.storage.len(), i);
			fixture.enqueue(i);
		}

		for i in 0..SIZE {
			assert_eq!(fixture[i], i);
		}

		for i in 0..(SIZE / 2) {
			fixture.enqueue(i + 100);
		}

		for i in 0..(SIZE / 2) {
			fixture.enqueue(i + 4);
		}

		for i in (SIZE / 2)..SIZE {
			fixture.enqueue(i + 100);
		}
	}

	#[test]
	fn dequeue() {
		let mut fixture = Fixture::default();

		for i in 0..SIZE {
			fixture.enqueue(i);
		}

		for i in 0..(SIZE / 2) {
			assert_eq!(fixture.dequeue().unwrap(), i);
		}

		for i in 0..(SIZE / 2) {
			assert_eq!(fixture[i], i + 4);
		}
	}
}
