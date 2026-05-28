use super::item::Item;
use std::rc::Rc;

#[derive(Debug)]
pub(super) struct Probe(Rc<Item>);

impl Probe {
	pub(super) fn new(item: Rc<Item>) -> Self {
		assert!(!item.is_dropped());
		Self(item)
	}
	pub fn id(&self) -> usize {
		self.0.id()
	}
	pub fn is_dropped(&self) -> bool {
		self.0.is_dropped()
	}
	pub fn mark_dropped(&self) {
		self.0.mark_dropped();
	}
}

#[cfg(test)]
mod tests {
	use super::super::item::Item;
	use super::*;
	#[test]
	fn new() {
		let fixture = Probe::new(Rc::new(Item::new(42)));
		assert_eq!(fixture.0.id(), 42);
		assert_eq!(fixture.0.is_dropped(), false);
	}

	#[test]
	#[should_panic]
	fn invalid_new() {
		let item = Item::new(42);
		item.mark_dropped();
		let _ = Probe::new(Rc::new(item));
	}

	#[test]
	fn id() {
		let fixture = Probe::new(Rc::new(Item::new(42)));
		assert_eq!(fixture.id(), 42);
	}

	#[test]
	fn is_dropped() {
		let fixture = Probe::new(Rc::new(Item::new(42)));
		assert_eq!(fixture.is_dropped(), false);
		fixture.0.mark_dropped();
		assert_eq!(fixture.is_dropped(), true);
	}

	#[test]
	fn mark_dropped() {
		let fixture = Probe::new(Rc::new(Item::new(42)));
		fixture.mark_dropped();
		assert_eq!(fixture.0.is_dropped(), true);
	}
}
