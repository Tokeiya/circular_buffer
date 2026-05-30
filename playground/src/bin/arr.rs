use circular_buffer::CircularBuffer;
use circular_buffer::fixed::*;

fn main() {
	let mut buff = Buffer::<usize, 8>::default();

	buff.enqueue(10);
}
