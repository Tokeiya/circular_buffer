use circular_buffer::CircularBuffer;
use circular_buffer::fixed::*;

fn main() {
	let mut buffer = Buffer::<usize, Pow2IndexCoordinator<16>, 16>::default();

	buffer.enqueue(10);
}
