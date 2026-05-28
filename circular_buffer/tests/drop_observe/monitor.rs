use super::item::Item;
use super::specimen::Specimen;
use crate::drop_observe::probe::Probe;
use std::cell::Cell;
use std::rc::Rc;
#[derive(Debug)]
pub struct Monitor(Rc<Item>, Cell<bool>);

impl Monitor {
	pub(super) fn new(item: Item) -> Self {
		Self(Rc::new(item), Cell::new(false))
	}
	pub fn id(&self) -> usize {
		self.0.id()
	}

	pub fn is_dropped(&self) -> bool {
		self.0.is_dropped()
	}

	pub fn payout_specimen(&self) -> Specimen {
		if self.1.get() {
			panic!("Probe is already paid out");
		}
		self.1.set(true);
		Specimen::new(Probe::new(self.0.clone()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::drop_observe::monitor_gen::MonitorGenerator;
	#[test]
	fn from_id_is_dropped() {
		let mut generator = MonitorGenerator::default();

		for i in 0..10 {
			let fixture = Monitor::from(generator.generate());
			assert_eq!(fixture.id(), i);
			assert_eq!(fixture.is_dropped(), false);
		}
	}

	#[test]
	fn probe() {
		let mut generator = MonitorGenerator::default();

		for i in 0..10 {
			let fixture = Monitor::from(generator.generate());
			let specimen = fixture.payout_specimen();
			assert_eq!(specimen.id(), i);
			assert_eq!(specimen.is_dropped(), false);
		}
	}

	#[test]
	#[should_panic]
	fn dupl_probe() {
		let fixture = Monitor::new(Item::new(42));
		let a = fixture.payout_specimen();
		fixture.payout_specimen();
	}
}
