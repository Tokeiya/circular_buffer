use std::marker::PhantomData;

pub struct Envelope<T>(Vec<T>);

struct IterMut<'a, T> {
	pointer: *mut T,
	len: usize,
	capacity: usize,
	_phantom: PhantomData<&'a mut T>,
}

impl<'a, T> IterMut<'a, T> {
	pub fn new(_: &'a mut Envelope<T>, pointer: *mut T, capacity: usize) -> Self {
		Self {
			pointer,
			len: 0,
			capacity,
			_phantom: PhantomData,
		}
	}
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}

fn main() {}
