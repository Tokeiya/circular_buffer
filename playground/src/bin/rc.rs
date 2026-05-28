use std::cell::Cell;
use std::rc::Rc;

pub mod inner {
	use std::cell::Cell;
	use std::rc::Rc;

	#[derive(Debug)]
	pub struct Item {
		id: usize,
		dropped: Cell<bool>,
	}

	impl Item {
		pub fn new(id: usize) -> Self {
			Self {
				id,
				dropped: Cell::new(false),
			}
		}

		pub fn is_dropped(&self) -> bool {
			self.dropped.get()
		}

		pub fn id(&self) -> usize {
			self.id
		}

		pub fn mark_dropped(&self) {
			if !self.is_dropped() {
				self.dropped.set(true);
			} else {
				panic!("Item already marked as dropped");
			}
		}
	}

	#[derive(Debug)]
	pub struct Probe {
		item: Rc<Item>,
	}

	impl Probe {
		pub fn new(id: usize) -> Self {
			Self {
				item: Rc::new(Item::new(id)),
			}
		}

		pub fn id(&self) -> usize {
			self.item.id
		}

		pub fn is_dropped(&self) -> bool {
			self.item.dropped.get()
		}

		pub fn mark_dropped(&self) {
			self.item.mark_dropped();
		}
	}

	impl Clone for Probe {
		fn clone(&self) -> Self {
			Self {
				item: self.item.clone(),
			}
		}
	}
}

use crate::inner::*;
fn main() {
	let p = Probe::new(42);
	dbg!(p);
}
