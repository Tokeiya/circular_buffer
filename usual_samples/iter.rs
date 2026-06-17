huse crate::drop_observe::Probe;
use std::borrow::Borrow;
pub struct Iter<A, E> {
	actual: A,
	expected: E,
}

fn assert_probe(actual: &Probe, expected: &Probe) {
	assert_eq!(actual.id(), expected.id());
	assert_eq!(actual.is_dropped(), expected.is_dropped());
}

impl<A, E> Iter<A, E> {
	pub fn new(actual: A, expected: E) -> Self {
		Self { actual, expected }
	}
}

impl<A, E> Iter<A, E>
where
	A: Iterator,
	E: Iterator,
	A::Item: Borrow<Probe>,
	E::Item: Borrow<Probe>,
{
	pub fn assert(self) {
		let mut actual = self.actual;
		let mut expected = self.expected;

		loop {
			match (actual.next(), expected.next()) {
				(Some(a), Some(e)) => {
					assert_probe(a.borrow(), e.borrow());
				}
				(None, None) => break,
				_ => panic!("actual and expected iterators have different lengths"),
			}
		}
	}
}
impl<A, E> Iterator for Iter<A, E>
where
	A: Iterator,
	E: Iterator,
	A::Item: Borrow<Probe>,
	E::Item: Borrow<Probe>,
{
	type Item = E::Item;

	fn next(&mut self) -> Option<Self::Item> {
		match (self.actual.next(), self.expected.next()) {
			(Some(a), Some(e)) => {
				assert_probe(a.borrow(), e.borrow());
				Some(e)
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
	use std::panic::catch_unwind;
	const SIZE: usize = 16;
	type Expected = super::super::expected::Expected<Probe, SIZE>;
	type Actual = Buffer<Probe, SIZE>;
	#[test]
	fn new() {
		let act = Actual::default();
		let exp = Expected::default();

		let fixture = Fixture::new(act.iter(), exp.iter());
		assert_eq!(fixture.actual.len(), 0);
		assert_eq!(fixture.expected.len(), 0);
	}

	#[test]
	fn assert() {
		let mut act_gen = MonitorGenerator::default();
		let mut exp_gen = MonitorGenerator::default();

		let mut act = Actual::default();
		let mut exp = Expected::default();

		let fixture = Fixture::new(act.iter(), exp.iter());
		fixture.assert();

		for _ in 0..SIZE {
			act.enqueue(act_gen.generate().payout_probe());
			exp.enqueue(exp_gen.generate().payout_probe());
		}

		let fixture = Fixture::new(act.iter(), exp.iter());
		fixture.assert();
	}

	#[test]
	#[should_panic]
	fn assert_failed() {
		let mut factory = MonitorGenerator::default();

		let mut act = Actual::default();
		let mut exp = Expected::default();

		act.enqueue(factory.generate().payout_probe());
		exp.enqueue(factory.generate().payout_probe());

		let fixture = Fixture::new(act.iter(), exp.iter());
		fixture.assert();
	}
}
