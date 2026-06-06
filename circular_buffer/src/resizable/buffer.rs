#[cfg(test)]
use std::{cell::Cell, rc::Rc};

use super::IndexCoordinator;

pub struct Buffer<T, C: IndexCoordinator> {
	#[cfg(test)]
	probe: Option<Rc<Cell<usize>>>,
	pub(super) storage: Vec<T>,
	pub(super) coordinator: C,
}

impl<T, C: IndexCoordinator> Buffer<T, C> {}
