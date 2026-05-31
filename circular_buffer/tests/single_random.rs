mod drop_observe;
mod process_sync;

use circular_buffer::fixed::*;
use circular_buffer::{Error, Result};
use drop_observe::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use process_sync::Expected;

const SIZE: usize = 64;
type Fixture = Buffer<Probe, SIZE>;

#[test]
fn enqueue_dequeue() {
	let seed = rand::rng().next_u64();
	dbg!(seed);
	let mut rng = ChaCha8Rng::seed_from_u64(seed);

	let mut buffer = Fixture::default();
	let mut facory = MonitorGenerator::default();
	let mut hash = std::collections::HashMap::<usize, Monitor>::new();
}
