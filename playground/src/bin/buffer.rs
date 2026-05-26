use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};
use std::sync::atomic::AtomicUsize;
static SEED: AtomicUsize = AtomicUsize::new(0);
pub struct Droppable(usize);

impl Drop for Droppable {
	fn drop(&mut self) {
		println!("Dropping:{}", self.0);
	}
}

impl Default for Droppable {
	fn default() -> Self {
		Self(SEED.fetch_add(1, std::sync::atomic::Ordering::SeqCst))
	}
}

struct Survey<T>([MaybeUninit<T>; 4], usize);

impl<T> Default for Survey<T> {
	fn default() -> Self {
		let arr: [MaybeUninit<T>; 4] = [const { MaybeUninit::uninit() }; 4];
		Self(arr, 0)
	}
}

impl<T> Drop for Survey<T> {
	fn drop(&mut self) {
		for i in 0..self.1 {
			unsafe { self.0[i].assume_init_drop() };
		}
	}
}

impl<T> Index<usize> for Survey<T> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		if index >= self.1 {
			panic!("Index out of bounds");
		}
		unsafe { self.0[index].assume_init_ref() }
	}
}

impl<T> IndexMut<usize> for Survey<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		if index >= self.1 {
			panic!("Index out of bounds");
		}
		unsafe { self.0[index].assume_init_mut() }
	}
}

impl<T> Survey<T> {
	pub fn push(&mut self, val: T) {
		if self.1 >= 4 {
			panic!("Too many elements");
		}
		self.1 += 1;
		self.0[self.1 - 1].write(val);
	}
}

fn main() {
	let mut arr = Survey::<Droppable>::default();

	arr.push(Droppable::default());
	arr.push(Droppable::default());
}
