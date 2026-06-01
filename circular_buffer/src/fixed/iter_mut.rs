use crate::IndexCoordinator;
use crate::fixed::Buffer;
use crate::fixed::index_coordinator::Pow2IndexCoordinator;

pub struct IterMut<'a, T, const N: usize> {
	head_ptr: *mut std::mem::MaybeUninit<T>,
	coordinator: Pow2IndexCoordinator<N>,
	_phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, T, const N: usize> IterMut<'a, T, N> {
	pub(super) fn new(buffer: &'a mut Buffer<T, N>) -> Self {
		Self {
			head_ptr: buffer.storage.as_mut_ptr(),
			coordinator: buffer.coordinator.clone(),
			_phantom: std::marker::PhantomData,
		}
	}
}

impl<'a, T: 'a, const N: usize> Iterator for IterMut<'a, T, N> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.coordinator.len() == 0 {
			None
		} else {
			let ret = unsafe {
				let uninit_ref = &mut *self.head_ptr.add(self.coordinator.head_index().unwrap());
				Some(uninit_ref.assume_init_mut())
			};

			self.coordinator.dequeue_index().unwrap();

			ret
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.coordinator.len(), Some(self.coordinator.len()))
	}
}

impl<'a, T: 'a, const N: usize> ExactSizeIterator for IterMut<'a, T, N> {
	fn len(&self) -> usize {
		self.coordinator.len()
	}
}

impl<'a, T: 'a, const N: usize> DoubleEndedIterator for IterMut<'a, T, N> {
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.coordinator.len() == 0 {
			None
		} else {
			let ret = unsafe {
				let index = self.coordinator.tail_index().unwrap();

				let uninit_ref = &mut *self.head_ptr.add(index);
				Some(uninit_ref.assume_init_mut())
			};
			self.coordinator.pop_index().unwrap();
			ret
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::circular_buffer::CircularBuffer;
	use crate::fixed::buffer::Buffer;
	use crate::test_shared::*;

	const SIZE: usize = 8;
	type Buff = Buffer<usize, SIZE>;

	fn gen_sample() -> Buff {
		let mut buffer = Buff::default();
		for i in 0..8 {
			buffer.enqueue(i);
		}
		buffer
	}

	#[test]
	fn new_next() {
		let mut buff = gen_sample();

		let fixture = IterMut::new(&mut buff);

		for elem in fixture {
			*elem += 10;
		}

		for i in 0..SIZE {
			assert_eq!(buff[i], i + 10);
		}

		let mut fixture = IterMut::new(&mut buff);
		for _ in 0..SIZE {
			assert!(fixture.next().is_some());
		}

		assert!(fixture.next().is_none());
	}

	#[test]
	fn size_hint() {
		let mut buff = gen_sample();
		let mut fixture = IterMut::new(&mut buff);

		for i in 0..SIZE {
			assert_eq!(fixture.size_hint(), (SIZE - i, Some(SIZE - i)));
			fixture.next().unwrap();
		}

		assert_eq!(fixture.size_hint(), (0, Some(0)));
	}

	#[test]
	fn next_back() {
		let mut buff = gen_sample();

		let mut fixture = IterMut::new(&mut buff);

		for i in 0..SIZE {
			assert_eq!(*fixture.next_back().unwrap(), SIZE - 1 - i);
			assert_eq!(fixture.size_hint(), (SIZE - i - 1, Some(SIZE - i - 1)));
		}

		assert!(fixture.next_back().is_none());
	}

	#[test]
	fn complex_next() {
		let mut buff = gen_sample();

		for i in 10..100 {
			buff.enqueue(i);
		}

		print!("vec![");
		for i in IterMut::new(&mut buff) {
			print!("{},", i);
		}
		println!("]");

		let iter = IterMut::new(&mut buff);
		for i in iter.zip(92..100) {
			assert_eq!(i.0, &i.1);
		}
	}

	#[test]
	fn replace_via_iter() {
		let mut factory = MonitorGenerator::default();
		let may_be_dropped: [Monitor; SIZE] = std::array::from_fn(|_| factory.generate());
		let may_be_not_dropped: [Monitor; SIZE] = std::array::from_fn(|_| factory.generate());

		let mut fixture = Buffer::<Probe, SIZE>::default();

		for m in may_be_dropped.iter() {
			fixture.enqueue(m.payout_probe());
		}

		let iterator = IterMut::new(&mut fixture).enumerate();

		for (i, elem) in iterator {
			*elem = may_be_not_dropped[i].payout_probe();
		}

		assert!(may_be_dropped.iter().all(|m| m.is_dropped()));
		assert!(may_be_not_dropped.iter().all(|m| !m.is_dropped()));
	}
}
