mod drop_observe;
mod process_sync;

use overwrite_ring_buffer::resizable::{Buffer, CoordinatorSelector};
use drop_observe::*;
use process_sync::Expected;
use process_sync::test_process::{all_process_impl, enqueue_dequeue_impl};

type ActualFixture = Buffer<Probe, CoordinatorSelector>;
type ExpectedFixture<const N: usize> = Expected<Probe, N>;

#[test]
fn enqueue_dequeue_64() {
	enqueue_dequeue_impl::<_, _, 64>(
		ActualFixture::new(CoordinatorSelector::new(64).unwrap()),
		ExpectedFixture::<64>::default(),
	)
}

#[test]
fn enqueue_dequeue_127() {
	enqueue_dequeue_impl::<_, _, 127>(
		ActualFixture::new(CoordinatorSelector::new(127).unwrap()),
		ExpectedFixture::<127>::default(),
	)
}

#[test]
fn all_process_64() {
	all_process_impl::<_, _, 64>(
		ActualFixture::new(CoordinatorSelector::new(64).unwrap()),
		ExpectedFixture::<64>::default(),
	)
}

#[test]
fn all_process_127() {
	all_process_impl::<_, _, 127>(
		ActualFixture::new(CoordinatorSelector::new(127).unwrap()),
		ExpectedFixture::<127>::default(),
	)
}
