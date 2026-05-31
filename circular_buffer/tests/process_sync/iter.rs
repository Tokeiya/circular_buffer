use super::drop_observe::Probe;
use std::borrow::Borrow;
pub struct Iter<A, E> {
	actual: A,
	expected: E,
}

fn assert(actual: &Probe, expected: &Probe) {
	assert_eq!(actual.id(), expected.id());
	assert_eq!(actual.is_dropped(), expected.is_dropped());
}

impl<A, E> Iter<A, E> {
	pub fn new(actual: A, expected: E) -> Self {
		Self { actual, expected }
	}
}

impl<A, E> Iterator for Iter<A, E>
where
	A: Iterator,
	E: Iterator,
	A::Item: Borrow<Probe>,
	E::Item: Borrow<Probe>,
{
	type Item = A::Item;

	fn next(&mut self) -> Option<Self::Item> {
		match (self.actual.next(), self.expected.next()) {
			(Some(a), Some(e)) => {
				assert(a.borrow(), e.borrow());
				Some(a)
			}
			(None, None) => None,
			_ => panic!("actual and expected iterators have different lengths"),
		}
	}
}

#[cfg(test)]
mod test {
	use super::Iter as Fixture;
	use crate::drop_observe::*;
	use circular_buffer::CircularBuffer;
	use circular_buffer::fixed::*;
	const SIZE: usize = 16;
	type Expected = super::super::expected::Expected<Probe, SIZE>;
	type Actual = Buffer<Probe, SIZE>;
	#[test]
	fn foo() {
		let act = Actual::default();
		let exp = Expected::default();

		let mut fixture = Fixture::new(act.iter(), exp.iter());

		for elem in fixture {}
	}
}
