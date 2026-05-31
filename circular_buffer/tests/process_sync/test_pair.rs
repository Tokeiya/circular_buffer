use super::drop_observe::*;
use circular_buffer::CircularBuffer;
use std::collections::HashMap;
pub struct TestPair<A, E> {
	actual: A,
	expected: E,
}

impl<A: CircularBuffer<Probe> + Default, E: CircularBuffer<Probe> + Default> TestPair<A, E> {
	pub fn init() -> Self {
		Self {
			actual: A::default(),
			expected: E::default(),
		}
	}

	pub fn actual(&self) -> &A {
		&self.actual
	}

	pub fn expected(&self) -> &E {
		&self.expected
	}

	pub fn mut_actual(&mut self) -> &mut A {
		&mut self.actual
	}

	pub fn mut_expected(&mut self) -> &mut E {
		&mut self.expected
	}
}
