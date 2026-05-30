#![allow(dead_code)]

#[derive(Debug)]
struct Value(usize);

#[derive(Debug)]
struct Envelope<'a> {
	value: usize,
	_phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Envelope<'a> {
	fn new(value: &'a Value) -> Self {
		Self {
			value: value.0,
			_phantom: std::marker::PhantomData,
		}
	}
}

fn main() {
	let value = Value(42);
	let envelope = Envelope::new(&value);

	//drop(value);

	println!("{:?}", envelope);
}
