use super::buffer::Buffer;
use crate::IndexCoordinator;
use crate::fixed::pow2_index_coordinator::Pow2IndexCoordinator;
use std::iter::FusedIterator;

pub struct Iter<'a, T, const N: usize> {
	backend: &'a Buffer<T, N>,
	coordinator: Pow2IndexCoordinator<N>,
}

impl<'a, T, const N: usize> Iter<'a, T, N> {
	pub(super) fn new(item: &'a Buffer<T, N>) -> Self {
		Self {
			backend: item,
			coordinator: item.coordinator.clone(),
		}
	}
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.len() == 0 {
			None
		} else {
			let item = &self.backend.get_raw(self.coordinator.head_index().unwrap());
			self.coordinator.dequeue_index().unwrap();
			Some(item)
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.coordinator.len();
		(len, Some(len))
	}
}

impl<'a, T, const N: usize> ExactSizeIterator for Iter<'a, T, N> {
	fn len(&self) -> usize {
		self.coordinator.len()
	}
}

impl<'a, T, const N: usize> DoubleEndedIterator for Iter<'a, T, N> {
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.len() == 0 {
			None
		} else {
			let item = &self.backend[self.coordinator.tail_index().unwrap()];
			self.coordinator.pop_index().unwrap();
			Some(item)
		}
	}
}

impl<'a, T, const N: usize> FusedIterator for Iter<'a, T, N> {}

#[cfg(test)]
mod test {
	use super::*;
	use crate::circular_buffer::CircularBuffer;
	use crate::fixed::buffer::Buffer;

	const SIZE: usize = 8;
	type Fixture = Buffer<usize, SIZE>;

	fn fixture() -> Fixture {
		let mut fixture = Buffer::default();

		for i in 0..SIZE {
			fixture.enqueue(i)
		}

		fixture
	}

	#[test]
	fn next() {
		let buff = fixture();
		let mut fixture = Iter::new(&buff);

		for i in 0..SIZE {
			assert_eq!(fixture.next(), Some(&buff[i]));
		}

		for _ in 0..SIZE {
			assert_eq!(fixture.next(), None);
		}
	}

	#[test]
	fn len() {
		let buff = fixture();
		let mut fixture = Iter::new(&buff);

		for i in 0..SIZE {
			assert_eq!(fixture.len(), SIZE - i);
			fixture.next().unwrap();
		}

		assert_eq!(fixture.len(), 0);
	}

	#[test]
	fn next_back() {
		let buff = fixture();
		let mut fixture = Iter::new(&buff);

		for i in (0..SIZE).rev() {
			assert_eq!(fixture.next_back(), Some(&buff[i]));
		}

		assert!(fixture.next_back().is_none());
		assert_eq!(fixture.len(), 0);
	}

	#[test]
	fn complex() {
		let buff = fixture();
		let mut fixture = Iter::new(&buff);

		assert_eq!(fixture.next(), Some(&buff[0]));
		assert_eq!(fixture.next_back(), Some(&buff[7]));
		assert_eq!(fixture.next(), Some(&buff[1]));
		assert_eq!(fixture.next_back(), Some(&buff[6]));
		assert_eq!(fixture.next(), Some(&buff[2]));
		assert_eq!(fixture.next_back(), Some(&buff[5]));
		assert_eq!(fixture.next(), Some(&buff[3]));
		assert_eq!(fixture.next_back(), Some(&buff[4]));

		assert_eq!(fixture.next(), None);
		assert_eq!(fixture.next_back(), None);
		assert_eq!(fixture.len(), 0);
	}

	#[test]
	fn size_hint() {
		let buff = fixture();
		let mut fixture = Iter::new(&buff);

		for i in (1..=SIZE).rev() {
			let expected = i;
			assert_eq!(fixture.size_hint(), (expected, Some(expected)));
			fixture.next().unwrap();
		}

		assert_eq!(fixture.size_hint(), (0, Some(0)));

		fixture = Iter::new(&buff);

		for i in (1..=SIZE).rev() {
			assert_eq!(fixture.size_hint(), (i, Some(i)));
			fixture.next_back().unwrap();
		}
		assert_eq!(fixture.size_hint(), (0, Some(0)));
	}

	#[test]
	fn complex_next() {
		let mut buff = fixture();

		for i in 10..100 {
			buff.enqueue(i);
		}

		print!("vec![");
		for i in buff.iter() {
			print!("{},", i);
		}
		println!("]");

		for i in buff.iter().zip(92..100) {
			assert_eq!(i.0, &i.1);
		}
	}
}
