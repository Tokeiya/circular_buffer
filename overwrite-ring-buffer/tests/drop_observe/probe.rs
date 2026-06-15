use super::item::Item;
use std::fmt::Debug;
use std::rc::Rc;

pub struct Probe(Rc<Item>, Option<Box<dyn FnOnce(Rc<Item>)>>);

impl Probe {
	pub(super) fn new(item: Rc<Item>) -> Self {
		assert!(!item.is_dropped());
		Self(item, None)
	}

	#[allow(dead_code)]
	pub(super) fn new_with_behaviour<F: FnOnce(Rc<Item>) + 'static>(
		item: Rc<Item>,
		callback: F,
	) -> Self {
		assert!(!item.is_dropped());
		Self(item, Some(Box::new(callback)))
	}
	pub fn id(&self) -> usize {
		self.0.id()
	}

	pub fn is_dropped(&self) -> bool {
		self.0.is_dropped()
	}
}

impl Debug for Probe {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Probe {{ id: {}, is_dropped: {} }}",
			self.id(),
			self.is_dropped()
		)
	}
}

impl Drop for Probe {
	fn drop(&mut self) {
		if let Some(callback) = self.1.take() {
			callback(self.0.clone());
		} else {
			self.0.mark_dropped();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::super::item::Item;
	use super::*;
	use std::cell::Cell;
	use std::mem::{drop as consume, ManuallyDrop};
	#[test]
	fn new() {
		let fixture = Probe::new(Rc::new(Item::new(42)));
		assert_eq!(fixture.0.id(), 42);
		assert_eq!(fixture.0.is_dropped(), false);
	}

	#[test]
	fn new_with_behaviour() {
		let item = Rc::new(Item::new(42));
		let flg = Rc::new(Cell::new(false));
		let observer = flg.clone();

		let fixture = Probe::new_with_behaviour(item.clone(), move |i| {
			assert_eq!(i.id(), 42);
			flg.set(true);
			i.mark_dropped();
		});

		assert_eq!(item.is_dropped(), false);
		std::mem::drop(fixture);
		assert_eq!(item.is_dropped(), true);
		assert_eq!(observer.get(), true);
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
		let fixture = ManuallyDrop::new(Probe::new(Rc::new(Item::new(42))));
		assert_eq!(fixture.is_dropped(), false);
		fixture.0.mark_dropped();
		assert_eq!(fixture.is_dropped(), true);
	}

	#[test]
	fn drop() {
		let item = Rc::new(Item::new(42));
		let probe = Probe::new(item.clone());
		consume(probe);
		assert_eq!(item.is_dropped(), true);
	}
}
