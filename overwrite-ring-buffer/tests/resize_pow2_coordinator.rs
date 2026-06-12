mod drop_observe;
mod process_sync;

use overwrite_ring_buffer::resizable::{Buffer, Pow2IndexCoordinator};
use drop_observe::*;
use process_sync::Expected;
use process_sync::test_process::{all_process_impl, enqueue_dequeue_impl};

const SIZE: usize = 64;
type ActualFixture = Buffer<Probe, Pow2IndexCoordinator>;
type ExpectedFixture = Expected<Probe, SIZE>;
#[test]
fn enqueue_dequeue() {
	enqueue_dequeue_impl::<ActualFixture, ExpectedFixture, SIZE>(
		ActualFixture::new(Pow2IndexCoordinator::try_new(SIZE).unwrap()),
		ExpectedFixture::default(),
	);
}

#[test]
fn all_process() {
	all_process_impl::<ActualFixture, ExpectedFixture, SIZE>(
		ActualFixture::new(Pow2IndexCoordinator::try_new(SIZE).unwrap()),
		ExpectedFixture::default(),
	)
}
