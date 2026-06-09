pub use super::IndexCoordinator;
use std::iter::FusedIterator;

pub struct Iter<'a, T, C: IndexCoordinator> {
	buff: &'a [T],
	coordinator: C,
}

impl<'a, T, C: IndexCoordinator> Iter<'a, T, C> {
	pub(crate) fn new(buff: &'a [T], coordinator: C) -> Self {
		Self { buff, coordinator }
	}
}

impl<'a, T, C: IndexCoordinator> Iterator for Iter<'a, T, C> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.coordinator.len() == 0 {
			None
		} else {
			let item = &self.buff[self.coordinator.head_index().unwrap()];
			self.coordinator.dequeue_index().unwrap();
			Some(item)
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.coordinator.len();
		(len, Some(len))
	}
}

impl<'a, T, C: IndexCoordinator> ExactSizeIterator for Iter<'a, T, C> {
	fn len(&self) -> usize {
		self.coordinator.len()
	}
}

impl<'a, T, C: IndexCoordinator> DoubleEndedIterator for Iter<'a, T, C> {
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.coordinator.len() == 0 {
			None
		} else {
			let item = &self.buff[self.coordinator.tail_index().unwrap()];
			self.coordinator.pop_index().unwrap();
			Some(item)
		}
	}
}

impl<'a, T, C: IndexCoordinator> FusedIterator for Iter<'a, T, C> {}

#[cfg(test)]
mod test {
	use super::*;
	use crate::fixed::GeneralIndexCoordinator;
	use crate::fixed::index_coordinator_test::IndexCoordinatorTestExtensions;
	use std::alloc::handle_alloc_error;
	use std::array::from_fn;

	const SIZE: usize = 8;
	const MASK: usize = SIZE - 1;

	type Coordinator = GeneralIndexCoordinator<SIZE>;

	fn gen_sample() -> [usize; SIZE] {
		from_fn(|i| i)
	}

	fn expected_real_to_virtual(capacity: usize, index: usize, head: usize) -> usize {
		(index + capacity - head) % capacity
	}

	fn expected_virtual_to_real(capacity: usize, index: usize, head: usize) -> usize {
		(index + head) % capacity
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
					*e += offset;
					*e &= MASK;
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
		let mut scr = gen_sample();
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
					assert_eq!(a, &scr[expected_virtual_to_real(SIZE, idx, h)]);
					idx = idx.saturating_sub(1);
				}
			}
		}
	}
}
