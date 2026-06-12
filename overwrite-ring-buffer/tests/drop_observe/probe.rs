use super::item::Item;
use std::rc::Rc;

#[derive(Debug)]
pub struct Probe(Rc<Item>, bool);

impl Probe {
	pub(super) fn new(item: Rc<Item>, panic_on_drop: bool) -> Self {
		assert!(!item.is_dropped());
		Self(item, panic_on_drop)
	}
	pub fn id(&self) -> usize {
		self.0.id()
	}

	pub fn is_dropped(&self) -> bool {
		self.0.is_dropped()
	}
}

impl Drop for Probe {
	fn drop(&mut self) {
		self.0.mark_dropped();
	}
}

#[cfg(test)]
mod tests {
	use super::super::item::Item;
	use super::*;
	use std::mem::{drop as consume, ManuallyDrop};
	#[test]
	fn new() {
		let fixture = Probe::new(Rc::new(Item::new(42)), false);
		assert_eq!(fixture.0.id(), 42);
		assert_eq!(fixture.0.is_dropped(), false);
	}

	#[test]
	#[should_panic]
	fn invalid_new() {
		let item = Item::new(42);
		item.mark_dropped();
		let _ = Probe::new(Rc::new(item), false);
	}

	#[test]
	fn id() {
		let fixture = Probe::new(Rc::new(Item::new(42)), false);
		assert_eq!(fixture.id(), 42);
	}

	#[test]
	fn is_dropped() {
		let fixture = ManuallyDrop::new(Probe::new(Rc::new(Item::new(42)), false));
		assert_eq!(fixture.is_dropped(), false);
		fixture.0.mark_dropped();
		assert_eq!(fixture.is_dropped(), true);
	}

	#[test]
	fn drop() {
		let item = Rc::new(Item::new(42));
		let probe = Probe::new(item.clone(), false);
		consume(probe);
		assert_eq!(item.is_dropped(), true);
	}

	#[test]
	fn panic_on_drop() {
		todo!();
	}
}
