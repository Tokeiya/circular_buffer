mod drop_observe;
mod process_sync;

use overwrite_ring_buffer::resizable::{Buffer, GeneralIndexCoordinator};
use drop_observe::*;
use process_sync::Expected;
use process_sync::test_process::{all_process_impl, enqueue_dequeue_impl};

const SIZE: usize = 65;
type ActualFixture = Buffer<Probe, GeneralIndexCoordinator>;
type ExpectedFixture = Expected<Probe, SIZE>;
#[test]
fn enqueue_dequeue() {
	enqueue_dequeue_impl::<ActualFixture, ExpectedFixture, SIZE>(
		ActualFixture::new(GeneralIndexCoordinator::try_new(SIZE).unwrap()),
		ExpectedFixture::default(),
	);
}

#[test]
fn all_process() {
	all_process_impl::<ActualFixture, ExpectedFixture, SIZE>(
		ActualFixture::new(GeneralIndexCoordinator::try_new(SIZE).unwrap()),
		ExpectedFixture::default(),
	)
}
