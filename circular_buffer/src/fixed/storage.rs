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
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::panic::{AssertUnwindSafe, catch_unwind};
	use std::sync::atomic::{AtomicUsize, Ordering};

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
			let _ = Dummy::default();
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
	fn push_len() {
		let mut fixture = Storage::<usize, 8>::default();

		for i in 0..8 {
			assert_eq!(fixture.len(), i);
			fixture.enqueue(i);
		}

		assert_eq!(fixture.len(), 8);
	}

	#[test]
	fn push_over_flow() {
		let mut fixture = Storage::<usize, 8>::default();

		for i in 0..8 {
			fixture.enqueue(i);
		}

		assert!(catch_unwind(AssertUnwindSafe(|| fixture.enqueue(8))).is_err());
	}

	#[test]
	fn index() {
		let mut fixture = Storage::<usize, 8>::default();

		for i in 0..8 {
			for j in 0..i {
				assert_eq!(j, fixture[j])
			}

			fixture.enqueue(i);
		}
	}

	#[test]
	fn index_out_of_range() {
		let mut fixture = Storage::<usize, 8>::default();

		for i in 0..8 {
			for j in i..8 {
				assert!(catch_unwind(AssertUnwindSafe(|| fixture[j])).is_err());
			}

			fixture.enqueue(i);
		}
	}

	#[test]
	fn index_mut() {
		let mut fixture = Storage::<usize, 8>::default();

		for i in 0..8 {
			for j in i..8 {
				assert!(catch_unwind(AssertUnwindSafe(|| fixture[j] = 42)).is_err())
			}
			fixture.enqueue(i);
			fixture[i] += 10;
			assert_eq!(fixture[i], i + 10);
		}
	}

	#[test]
	fn drop() {
		for i in 0..8 {
			{
				let mut fixture = Storage::<Dummy, 8>::default();
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
		let mut fixture = Storage::<Dummy, 8>::default();
		fixture.enqueue(Dummy);

		{
			fixture[0] = Dummy;
		}

		assert_count(1);
	}
}
