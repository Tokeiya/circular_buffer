#![cfg(test)]
use super::index_coordinator::IndexCoordinator;
use crate::error::*;
use std::assert_matches;

fn expected_real_to_virtual(capacity: usize, index: usize, head: usize) -> usize {
	(index + capacity - head) % capacity
}

fn expected_virtual_to_real(capacity: usize, index: usize, head: usize) -> usize {
	(index + head) % capacity
}

pub(super) trait IndexCoordinatorTestExtension: IndexCoordinator + Sized {
	fn fixture(capacity: usize) -> Self;
	fn mut_capacity(&mut self) -> &mut usize;
	fn mut_head(&mut self) -> &mut usize;
	fn mut_len(&mut self) -> &mut usize;

	//noinspection DuplicatedCode
	fn head_index(capacity: usize) {
		let mut fixture = Self::fixture(capacity);
		for (h, l) in (0..capacity).flat_map(|h| (0..=capacity).map(move |l| (h, l))) {
			*fixture.mut_head() = h;
			*fixture.mut_len() = l;

			if l == 0 {
				assert_matches!(fixture.head_index(), Err(Error::Empty));
			} else {
				assert_eq!(fixture.head_index().unwrap(), h);
			}
		}
	}

	fn tail_index(capacity: usize) {
		let mut fixture = Self::fixture(capacity);
	}

	fn enqueue_index(capacity: usize, iteration: usize) {
		todo!()
	}

	fn dequeue_index(capacity: usize) {
		todo!()
	}
}
