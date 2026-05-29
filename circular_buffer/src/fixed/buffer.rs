use super::index_coordinator::IndexCoordinator;
use super::iter::Iter;
use crate::circular_buffer::CircularBuffer;
use std::mem::MaybeUninit;
use std::ops::{Index,IndexMut};

pub struct Buffer<T, const N: usize> {
	storage: [MaybeUninit<T>; N],
	coordinator: IndexCoordinator<N>,
}

impl<T, const N: usize> Default for Buffer<T, N> {
	fn default() -> Self {
		Self {
			storage: [const { MaybeUninit::uninit() }; N],
			coordinator: IndexCoordinator::new(),
		}
	}
}

impl<T, const N: usize> Index<usize> for Buffer<T, N> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		match self.coordinator.virtual_to_real(index) {
			Ok(i) => unsafe { self.storage[i].assume_init_ref() },
			Err(_) => panic!("Index out of bounds"),
		}
	}
}

impl<T, const N: usize> IndexMut<usize> for Buffer<T, N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		match self.coordinator.virtual_to_real(index) {
			Ok(i) => unsafe { self.storage[i].assume_init_mut() },
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

	fn enqueue(&mut self, item: T) {
		if self.len() < self.capacity() {
			self.storage[self.len()].write(item);
			self.coordinator.enqueue_index();
		} else {
			let index = self.coordinator.virtual_to_real(0).unwrap();
			unsafe {
				self.storage[index].assume_init_drop();
				self.storage[index].write(item);
			};

			self.coordinator.enqueue_index();
		}
	}

	fn dequeue(&mut self) -> Option<T> {
		if self.coordinator.len()==0{
			None
		}else{
			let index = self.coordinator.virtual_to_real(0).unwrap();
			let ret=unsafe {
				std::mem::replace(&mut self.storage[index], std::mem::MaybeUninit::uninit()).assume_init()
			};
			
			self.coordinator.dequeue_index().unwrap();
			
			Some(ret)
		}
	}

	fn iter(&self) -> Self::Iter<'_> {
		Iter::new(self)
	}
	fn len(&self) -> usize {
		//debug_assert_eq!(self.storage.len(), self.coordinator.len());
		self.coordinator.len()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::panic::catch_unwind;

	const SIZE: usize = 8;
	type Fixture = Buffer<u8, SIZE>;

	#[test]
	fn default() {
		let fixture = Fixture::default();
		// assert_eq!(fixture.storage.len(), 0);
		assert_eq!(fixture.coordinator.len(), 0);
	}

	#[test]
	fn capacity() {
		let fixture = Fixture::default();
		assert_eq!(fixture.capacity(), 8);
	}

	#[test]
	fn push_index_len() {
		let mut fixture = Fixture::default();
		assert!(catch_unwind(|| _ = fixture[0]).is_err());

		for i in 0..SIZE {
			fixture.enqueue(i as u8);
			assert_eq!(fixture.len(), i + 1);
		}

		for i in 0..SIZE {
			assert_eq!(fixture[i], i as u8);
		}

		assert!(catch_unwind(|| _ = fixture[SIZE]).is_err());

		const OFFSET: u8 = 100;

		for i in 0..SIZE {
			fixture.enqueue(i as u8 + OFFSET);
			assert_eq!(fixture[7], i as u8 + OFFSET);
			if i == 7 {
				assert_eq!(fixture[0], 100);
			} else {
				assert_eq!(fixture[0], (i as u8) + 1);
			}
		}

		for i in 0..SIZE {
			assert_eq!(fixture[i], (i as u8) + OFFSET);
			assert_eq!(fixture.len(), SIZE);
		}

		fixture.enqueue(42);
		assert_eq!(fixture[7], 42);
		assert_eq!(fixture[0], 101);
	}
	
	#[test]
	fn index_mut() {
		let mut fixture = Fixture::default();
		assert!(catch_unwind(|| _ = fixture[0]).is_err());
		
		for i in 0..SIZE{
			fixture.enqueue(i as u8);
			assert_eq!(fixture[i], i as u8);
		}
		
		for i in 0..SIZE{
			fixture[i] = i as u8 + 10;
			assert_eq!(fixture[i], i as u8 + 10);
		}
		
	}

	#[test]
	fn iter() {
		let mut fixture = Fixture::default();

		for i in 0..SIZE {
			{
				assert_eq!(fixture.iter().len(), i);
				let mut iter = fixture.iter();
				for exp in 0..i {
					assert_eq!(*iter.next().unwrap(), exp as u8);
				}
			}

			fixture.enqueue(i as u8);
		}

		assert_eq!(fixture.iter().len(), SIZE);

		for (e, i) in fixture.iter().enumerate() {
			assert_eq!(*i, e as u8);
		}
	}
	
	#[test]
	fn dequeue() {
		let mut fixture = Fixture::default();
		assert!(fixture.dequeue().is_none());
		
		for i in 0..SIZE {
			fixture.enqueue(i as u8);
		}
		
		
		for i in 0..SIZE {
			let act=fixture.dequeue().unwrap();
			assert_eq!(act, i as u8);
		}
		
		assert_eq!(fixture.dequeue(), None);
	}
}
