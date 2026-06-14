use super::item::Item;
use super::probe::Probe;
use std::cell::Cell;
use std::rc::Rc;
#[derive(Debug)]
pub struct Monitor(Rc<Item>, Cell<bool>);

impl Monitor {
	pub(super) fn new(item: Item) -> Self {
		Self(Rc::new(item), Cell::new(false))
	}

	pub fn payout_probe(&self) -> Probe {
		if self.1.get() {
			panic!("Probe is already paid out");
		}
		self.1.set(true);
		Probe::new(self.0.clone())
	}

	pub fn id(&self) -> usize {
		self.0.id()
	}

	pub fn is_dropped(&self) -> bool {
		self.0.is_dropped()
	}
}

#[cfg(test)]
mod tests {
	use super::super::monitor_gen::MonitorGenerator;
	use super::*;
	#[test]
	fn from_id_is_dropped() {
		let mut generator = MonitorGenerator::default();

		for i in 0..10 {
			let fixture = generator.generate();
			assert_eq!(fixture.id(), i);
			assert!(!fixture.is_dropped());
		}
	}

	#[test]
	fn probe() {
		let mut generator = MonitorGenerator::default();

		for i in 0..10 {
			let fixture = generator.generate();
			let specimen = fixture.payout_probe();
			assert_eq!(specimen.id(), i);
			assert!(!specimen.is_dropped());
		}
	}

	#[test]
	fn is_dropped() {
		let fixture = Monitor::new(Item::new(42));

		assert!(!fixture.is_dropped());

		let probe = fixture.payout_probe();

		assert!(!fixture.is_dropped());

		std::mem::drop(probe);

		assert!(fixture.is_dropped());
	}

	#[test]
	#[should_panic]
	fn duplicate_probe() {
		let fixture = Monitor::new(Item::new(42));
		let _ = fixture.payout_probe();
		fixture.payout_probe();
	}
}
