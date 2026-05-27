use circular_buffer::fixed::*;
use circular_buffer::*;
fn main() {
	let mut buffer = Buffer::<u8, 16>::default();
	buffer.push(1);
}
