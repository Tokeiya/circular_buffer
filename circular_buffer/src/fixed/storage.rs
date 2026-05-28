use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};

pub struct Storage<T, const N: usize> {
	storage: [MaybeUninit<T>; N],
	len: usize,
}

impl<T, const N: usize> Default for Storage<T, N> {
	fn default() -> Self {
		Self {
			storage: [const { MaybeUninit::uninit() }; N],
			len: 0,
		}
	}
}

impl<T, const N: usize> Index<usize> for Storage<T, N> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		if index >= self.len {
			panic!("index out of range");
		}
		unsafe { self.storage[index].assume_init_ref() }
	}
}

impl<T, const N: usize> IndexMut<usize> for Storage<T, N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		if index >= self.len {
			panic!("index out of range");
		}
		unsafe { self.storage[index].assume_init_mut() }
	}
}

impl<T, const N: usize> Drop for Storage<T, N> {
	fn drop(&mut self) {
		for i in 0..self.len {
			unsafe {
				self.storage[i].assume_init_drop();
			}
		}
	}
}

impl<T, const N: usize> Storage<T, N> {
	pub fn enqueue(&mut self, value: T) {
		if self.len >= N {
			panic!("storage is full");
		}

		self.storage[self.len].write(value);
		self.len += 1;
	}

	pub fn dequeue(&mut self) -> Option<T> {
		todo!()
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn is_empty(&self) -> bool {
		self.len == 0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::prelude::*;
	use std::collections::VecDeque;
	use std::panic::{AssertUnwindSafe, catch_unwind};
	use std::sync::atomic::{AtomicUsize, Ordering};

	const BASE: usize = 8;

	thread_local! {
		static COUNT:AtomicUsize=const { AtomicUsize::new(0) };
	}

	struct Dummy;
	impl Drop for Dummy {
		fn drop(&mut self) {
			COUNT.with(|c| c.fetch_add(1, Ordering::Relaxed));
		}
	}

	impl Default for Dummy {
		fn default() -> Self {
			Self
		}
	}

	fn assert_count(cnt: usize) {
		assert_eq!(COUNT.with(|c| c.load(Ordering::SeqCst)), cnt);
	}

	fn reset_count() {
		COUNT.with(|c| c.store(0, Ordering::Relaxed));
	}

	#[test]
	fn verify() {
		{
			let _ = Dummy;
		}

		assert_eq!(
			COUNT.with(|c| c.load(std::sync::atomic::Ordering::SeqCst)),
			1
		);

		assert_count(1);
	}

	#[test]
	fn default() {
		{
			let fixture = Storage::<Dummy, 10>::default();
			assert_eq!(fixture.len, 0);
		}

		assert_count(0)
	}

	#[test]
	fn enqueue_len() {
		let mut fixture = Storage::<usize, BASE>::default();

		for i in 0..BASE {
			assert_eq!(fixture.len(), i);
			fixture.enqueue(i);
		}

		assert_eq!(fixture.len(), BASE);
	}

	#[test]
	fn enqueue_over_flow() {
		let mut fixture = Storage::<usize, BASE>::default();

		for i in 0..BASE {
			fixture.enqueue(i);
		}

		assert!(catch_unwind(AssertUnwindSafe(|| fixture.enqueue(BASE))).is_err());
	}

	#[test]
	fn index() {
		let mut fixture = Storage::<usize, BASE>::default();

		for i in 0..BASE {
			for j in 0..i {
				assert_eq!(j, fixture[j])
			}

			fixture.enqueue(i);
		}
	}

	#[test]
	fn index_out_of_range() {
		let mut fixture = Storage::<usize, BASE>::default();

		for i in 0..BASE {
			for j in i..BASE {
				assert!(catch_unwind(AssertUnwindSafe(|| fixture[j])).is_err());
			}

			fixture.enqueue(i);
		}
	}

	#[test]
	fn index_mut() {
		let mut fixture = Storage::<usize, BASE>::default();

		for i in 0..BASE {
			for j in i..BASE {
				assert!(catch_unwind(AssertUnwindSafe(|| fixture[j] = 42)).is_err())
			}
			fixture.enqueue(i);
			fixture[i] += 10;
			assert_eq!(fixture[i], i + 10);
		}
	}

	#[test]
	fn drop() {
		for i in 0..BASE {
			{
				let mut fixture = Storage::<Dummy, BASE>::default();
				for _ in 0..i {
					fixture.enqueue(Dummy::default());
				}
			}

			assert_count(i);
			reset_count();
		}
	}

	#[test]
	fn swap() {
		let mut fixture = Storage::<Dummy, BASE>::default();
		fixture.enqueue(Dummy);

		{
			fixture[0] = Dummy;
		}

		assert_count(1);
	}

	#[test]
	fn is_empty() {
		let mut fixture = Storage::<usize, BASE>::default();
		assert!(fixture.is_empty());

		for i in 0..BASE {
			fixture.enqueue(i);
			assert!(!fixture.is_empty());
		}
	}

	#[test]
	fn dequeue() {
		let mut fixture = Storage::<usize, BASE>::default();
		assert!(fixture.dequeue().is_none());

		for i in 0..BASE {
			fixture.enqueue(i);
		}

		for i in (0..BASE).rev() {
			assert_eq!(fixture.dequeue().unwrap(), i);
		}
		assert!(fixture.dequeue().is_none());
	}

	#[test]
	fn dequeue_drop() {
		fn test_body(n: usize) {
			let mut fixture = Storage::<Dummy, BASE>::default();
			for _ in 0..n {
				fixture.enqueue(Dummy);
			}

			fixture.dequeue().unwrap();
		}

		for i in 0..BASE {
			COUNT.with(|c| c.fetch_add(0, Ordering::Relaxed));
			test_body(i);
			COUNT.with(|c| assert_eq!(c.load(Ordering::Relaxed), 0));
		}
	}
}
