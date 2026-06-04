use playground::*;
use std::mem::MaybeUninit;

const SIZE: usize = 8;

pub struct Envelope([MaybeUninit<Probe>; SIZE]);

impl Envelope {
	pub fn new(monitor: &[Monitor; SIZE]) -> Self {
		let arr: [_; SIZE] = std::array::from_fn(|i| MaybeUninit::new(monitor[i].payout_probe()));
		Self(arr)
	}
}

impl Drop for Envelope {
	fn drop(&mut self) {
		for i in 0..SIZE {
			unsafe { self.0[i].assume_init_drop() };
		}
	}
}

fn main() {
	const SIZE: usize = 8;

	let mut factory = MonitorGenerator::default();
	let monitor: [Monitor; SIZE] = std::array::from_fn(|_| factory.generate());

	let envelope = Envelope::new(&monitor);
	drop(envelope);

	for (i, m) in monitor.iter().enumerate() {
		println!("[{i}].is_dropped: {}", m.is_dropped());
	}
}
