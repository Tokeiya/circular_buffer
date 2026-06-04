mod drop_observe;
mod process_sync;

use crate::drop_observe::Probe;
use crate::process_sync::Expected;
use crate::process_sync::test_process::{all_process_impl, enqueue_dequeue_impl};
use circular_buffer::fixed::Buffer;
use circular_buffer::fixed::IndexCoordinator;

const SIZE: usize = 100;
type ActualFixture = Buffer<Probe, IndexCoordinator<SIZE>, SIZE>;
type ExpectedFixture = Expected<Probe, SIZE>;

#[test]
fn enqueue_dequeue() {
	enqueue_dequeue_impl::<ActualFixture, ExpectedFixture, SIZE>();
}

#[test]
fn all_process() {
	all_process_impl::<ActualFixture, ExpectedFixture, SIZE>();
}
