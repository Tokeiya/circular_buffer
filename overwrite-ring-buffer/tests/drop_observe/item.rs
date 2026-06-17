use std::cell::Cell;

type Flag = Cell<bool>;

#[derive(Debug)]
pub struct Item {
	id: usize,
	flag: Flag,
}

impl Item {
	pub(super) fn new(id: usize) -> Self {
		Self {
			id,
			flag: Flag::new(false),
		}
	}
	pub fn id(&self) -> usize {
		self.id
	}

	pub fn is_dropped(&self) -> bool {
		self.flag.get()
	}

	pub fn mark_dropped(&self) {
		if self.flag.get() {
			panic!("Item is already dropped:{}", self.id);
		}
		self.flag.set(true);
	}
}

#[cfg(test)]
mod tests {
	use super::super::item::Item;

	#[test]
	fn new() {
		for i in 0..128 {
			let fixture = Item::new(i);
			assert_eq!(fixture.id, i);
			assert!(!fixture.flag.get());
		}
	}

	#[test]
	fn id() {
		for i in 0..128 {
			let fixture = Item::new(i);
			assert_eq!(fixture.id(), i);
		}
	}

	#[test]
	fn is_dropped() {
		let fixture = Item::new(0);
		assert!(!fixture.is_dropped());
		fixture.flag.set(true);
		assert!(fixture.is_dropped());
	}

	#[test]
	fn mark_dropped() {
		let fixture = Item::new(0);
		fixture.mark_dropped();
		assert!(fixture.flag.get());
	}

	#[test]
	#[should_panic]
	fn mark_dropped_dupl() {
		let fixture = Item::new(0);
		fixture.mark_dropped();
		fixture.mark_dropped();
	}
}
