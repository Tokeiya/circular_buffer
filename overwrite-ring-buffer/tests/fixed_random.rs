mod drop_observe;
mod process_sync;

use crate::drop_observe::Probe;
use crate::process_sync::Expected;
use crate::process_sync::test_process::{all_process_impl, enqueue_dequeue_impl};
use overwrite_ring_buffer::fixed::Buffer;
use overwrite_ring_buffer::fixed::GeneralIndexCoordinator;

const SIZE: usize = 100;
type ActualFixture = Buffer<Probe, GeneralIndexCoordinator<SIZE>, SIZE>;
type ExpectedFixture = Expected<Probe, SIZE>;

#[test]
fn enqueue_dequeue() {
	enqueue_dequeue_impl::<ActualFixture, ExpectedFixture, SIZE>(
		ActualFixture::default(),
		ExpectedFixture::default(),
	);
}

#[test]
fn all_process() {
	all_process_impl::<ActualFixture, ExpectedFixture, SIZE>(
		ActualFixture::default(),
		ExpectedFixture::default(),
	);
}
