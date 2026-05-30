use circular_buffer::CircularBuffer;
use circular_buffer::fixed::Buffer;
use std::collections::VecDeque;
fn main() {
	let mut buffer = Buffer::<usize, 8>::default();

	for i in 0..8 {
		buffer.enqueue(i);
	}

	for i in 0..4 {
		println!("{:?}", buffer.dequeue());
	}
}
