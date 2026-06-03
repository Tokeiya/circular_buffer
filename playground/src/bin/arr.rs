use circular_buffer::CircularBuffer;
use circular_buffer::fixed::*;

fn main() {
	let mut tmp = 0u8.wrapping_sub(1).wrapping_add(1);
	dbg!(tmp);
	dbg!(u8::MAX - tmp);
	tmp %= 10;
	dbg!(tmp);
}
