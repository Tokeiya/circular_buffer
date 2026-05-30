mod drop_observe;

use circular_buffer::CircularBuffer;
use circular_buffer::fixed::Buffer;
use drop_observe::{Monitor, MonitorGenerator, Probe};

const SIZE: usize = 8;
type Fixture = Buffer<Probe, SIZE>;

#[test]
fn drop() {
	let mut factory = MonitorGenerator::default();
	let monitor: [Monitor; SIZE] = std::array::from_fn(|_| factory.generate());

	let mut fixture = Fixture::default();
	for i in 0..SIZE {
		fixture.enqueue(monitor[i].payout_probe())
	}
	std::mem::drop(fixture);

	assert!(monitor.iter().all(|m| m.is_dropped()))
}

#[test]
fn overwrite() {
	const NUM: usize = 12;

	let mut factory = MonitorGenerator::default();
	let monitor: [Monitor; NUM] = std::array::from_fn(|_| factory.generate());

	let mut fixture = Fixture::default();
	for m in monitor.iter() {
		fixture.enqueue(m.payout_probe());
	}

	for m in monitor.iter().take(4) {
		assert!(m.is_dropped())
	}

	for m in monitor.iter().skip(4) {
		assert!(!m.is_dropped())
	}
}
