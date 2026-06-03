mod drop_observe;
mod process_sync;

use circular_buffer::fixed::Buffer;
use drop_observe::*;
use process_sync::Expected;
use process_sync::test_process::{all_process_impl, enqueue_dequeue_impl};

const ITERATION: usize = 8192;
const SIZE: usize = 64;
type ActualFixture = Buffer<Probe, SIZE>;
type ExpectedFixture = Expected<Probe, SIZE>;

#[test]
fn enqueue_dequeue() {
	enqueue_dequeue_impl::<ActualFixture, ExpectedFixture>();
}

#[test]
fn all_process() {
	all_process_impl::<ActualFixture, ExpectedFixture>();
}
