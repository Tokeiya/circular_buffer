use overwrite_ring_buffer::CircularBuffer;
use overwrite_ring_buffer::fixed::{Buffer, Pow2IndexCoordinator};
use playground::*;
use std::array::from_fn;
use std::panic::{AssertUnwindSafe, catch_unwind};

fn main() {
	let mut generator = MonitorGenerator::default();
	let monitor: [Monitor; 16] = from_fn(|_| generator.generate());
	let mut buffer = Buffer::<Probe, Pow2IndexCoordinator<16>, 16>::default();

	for elem in monitor.iter() {
		if elem.id() != 8 && elem.id() != 7 {
			buffer.enqueue(elem.payout_probe_with_behaviour(|item| {
				println!("{} dropped", item.id());
				item.mark_dropped();
			}));
		} else {
			buffer.enqueue(elem.payout_probe_with_behaviour(|item| {
				panic!("{} Scheduled", item.id());
			}))
		}
	}

	_ = catch_unwind(AssertUnwindSafe(|| drop(buffer)));
}
