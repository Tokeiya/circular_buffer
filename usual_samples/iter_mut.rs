use crate::drop_observe::Probe;
use std::borrow::BorrowMut;

pub struct IterMut<A, E> {
	actual: A,
	expected: E,
}

fn assert(act: &impl BorrowMut<Probe>, exp: &impl BorrowMut<Probe>) {
	let a = act.borrow();
	let e = exp.borrow();

	assert_eq!(a.id(), e.id());
	assert_eq!(a.is_dropped(), e.is_dropped());
}

impl<A, E> IterMut<A, E> {
	pub fn new(actual: A, expected: E) -> Self {
		Self { actual, expected }
	}
}

impl<A, E> Iterator for IterMut<A, E>
where
	A: Iterator,
	E: Iterator,
	A::Item: BorrowMut<Probe>,
	E::Item: BorrowMut<Probe>,
{
	type Item = E::Item;

	fn next(&mut self) -> Option<Self::Item> {
		match (self.actual.next(), self.expected.next()) {
			(Some(a), Some(e)) => {
				assert(&a, &e);
				Some(e)
			}
			(None, None) => None,
			_ => panic!("actual and expected iterators have different lengths"),
		}
	}
}
