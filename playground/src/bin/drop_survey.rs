use playground::*;
use std::mem::MaybeUninit;

struct Envelope<T>(MaybeUninit<T>);

impl<T> Envelope<T> {
	pub fn new(value: T) -> Self {
		Self(MaybeUninit::new(value))
	}

	pub fn mut_ref(&mut self) -> &mut T {
		unsafe { self.0.assume_init_mut() }
	}
}

fn main() {
	let mut generator = MonitorGenerator::default();
	let monitor_a = generator.generate();
	let monitor_b = generator.generate();

	let mut envelope = Envelope::new(monitor_a.payout_probe());
	*envelope.mut_ref() = monitor_b.payout_probe();

	println!("{}", monitor_a.is_dropped());
}
