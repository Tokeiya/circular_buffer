use crate::drop_observe::Probe;

pub struct IterMut<A, E> {
	actual: A,
	expected: E,
}

impl<A, E> IterMut<A, E> {
	pub fn new(actual: A, expected: E) -> Self {
		Self { actual, expected }
	}
}

impl<'a, A: Iterator<Item = &'a mut Probe>, E: Iterator<Item = &'a mut Probe>> Iterator
	for IterMut<A, E>
{
	type Item = &'a mut Probe;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}
