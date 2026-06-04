use circular_buffer::CircularBuffer;
use circular_buffer::fixed::*;
use std::ops::DerefMut;

fn main() {
	let mut b = Box::new(Buffer::<usize, Pow2IndexCoordinator<64>, 64>::default());
	b.enqueue(100);
	b.deref_mut().enqueue(200);
}
