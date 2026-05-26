use super::index_coordinator::IndexCoordinator;
use super::iter::Iter;
use super::storage::Storage;
use crate::circular_buffer::CircularBuffer;
use std::ops::Index;

pub struct Buffer<T, const N: usize> {
	storage: Storage<T, N>,
	coordinator: IndexCoordinator<N>,
}

impl<T, const N: usize> Default for Buffer<T, N> {
	fn default() -> Self {
		Self {
			storage: Storage::default(),
			coordinator: IndexCoordinator::new(),
		}
	}
}

impl<T, const N: usize> Index<usize> for Buffer<T, N> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		match self.coordinator.virtual_to_real(index) {
			Ok(i) => &self.storage[i],
			Err(_) => panic!("Index out of bounds"),
		}
	}
}

impl<T, const N: usize> CircularBuffer<T> for Buffer<T, N> {
	type Iter<'a>
		= Iter<'a, T, N>
	where
		T: 'a,
		Self: 'a;

	fn capacity(&self) -> usize {
		N
	}

	fn push(&mut self, item: T) {
		if self.len() < self.capacity() {
			self.storage.push(item);
			self.coordinator.push_index();
		} else {
			self.storage[self.coordinator.virtual_to_real(0).unwrap()] = item;
			self.coordinator.push_index();
		}
	}

	fn iter(&self) -> Self::Iter<'_> {
		todo!()
	}
	fn len(&self) -> usize {
		self.coordinator.len()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::panic::{AssertUnwindSafe, catch_unwind};
	type Fixture = Buffer<u8, 8>;

	#[test]
	fn default() {
		let fixture = Buffer::<u8, 8>::default();
		assert_eq!(fixture.storage.len(), 0);
		assert_eq!(fixture.coordinator.len(), 0);
	}

	#[test]
	fn capacity() {
		let fixture = Buffer::<u8, 8>::default();
		assert_eq!(fixture.capacity(), 8);
	}

	#[test]
	fn push_index() {
		let mut fixture = Buffer::<u8, 8>::default();
		assert!(catch_unwind(|| _ = fixture[0]).is_err());

		for i in 0..8 {
			fixture.push(i);
			assert_eq!(fixture.len(), (i + 1).into());
		}

		for i in 0..8 {
			assert_eq!(fixture[i], i as u8);
		}

		assert!(catch_unwind(|| _ = fixture[8]).is_err());

		for i in 10..18 {
			fixture.push(i);
		}

		for i in 0..8 {
			assert_eq!(fixture[i], (i as u8) + 10);
			assert_eq!(fixture.len(), 8);
		}
	}
}
