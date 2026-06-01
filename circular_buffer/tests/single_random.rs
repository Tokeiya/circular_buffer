mod drop_observe;
mod process_sync;

use circular_buffer::CircularBuffer;
use circular_buffer::fixed::Buffer;
use drop_observe::*;
use process_sync::Expected;
use process_sync::test_pair::TestPair;
use process_sync::test_process::{all_process_impl, enqueue_dequeue_impl};
use rand::RngExt;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

const ITERATION: usize = 8192;
const SIZE: usize = 64;
type ActualFixture = Buffer<Probe, SIZE>;
type ExpectedFixture = Expected<Probe, SIZE>;

#[derive(Eq, PartialEq)]
enum Process {
	Enqueue,
	Dequeue,
	IndexMut,
	IterMut,
}

#[test]
fn enqueue_dequeue() {
	enqueue_dequeue_impl::<ActualFixture, ExpectedFixture>();
}

#[test]
fn all_process() {
	all_process_impl::<ActualFixture, ExpectedFixture>();
}
