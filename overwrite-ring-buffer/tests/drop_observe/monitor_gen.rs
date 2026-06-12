use super::item::Item;
use super::monitor::Monitor;
pub struct MonitorGenerator(usize);

impl MonitorGenerator {
	pub fn new(initial: usize) -> Self {
		Self(initial)
	}

	pub fn generate(&mut self, panic_on_drop: bool) -> Monitor {
		let id = self.0;
		self.0 += 1;
		Monitor::new(Item::new(id), panic_on_drop)
	}
}

impl Default for MonitorGenerator {
	fn default() -> Self {
		Self::new(0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn new() {
		for i in 0..10 {
			let fixture = MonitorGenerator::new(i);
			assert_eq!(fixture.0, i);
		}
	}

	#[test]
	fn generate() {
		let mut fixture = MonitorGenerator::new(0);
		for i in 0..10 {
			let monitor = fixture.generate(false);
			assert_eq!(monitor.id(), i);
		}
	}

	#[test]
	fn default() {
		let fixture = MonitorGenerator::default();
		assert_eq!(fixture.0, 0);
	}
}
