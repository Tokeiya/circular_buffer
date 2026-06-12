use crate::IndexCoordinator;
use std::mem::MaybeUninit;
pub(crate) struct DropGuard<'a, T, C: IndexCoordinator> {
	storage: &'a mut [MaybeUninit<T>],
	coordinator: C,
}

impl<'a, T: 'a, C: IndexCoordinator> DropGuard<'a, T, C> {
	pub(crate) fn new(storage: &'a mut [MaybeUninit<T>], coordinator: C) -> Self {
		todo!()
	}

	pub(crate) fn drop_next(&mut self) -> bool {
		todo!()
	}
}

impl<'a, T: 'a, C: IndexCoordinator> Drop for DropGuard<'a, T, C> {
	fn drop(&mut self) {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_shared::*;
	use std::panic::{catch_unwind, AssertUnwindSafe};
	
	#[test]
	fn fo() {
		let mut generator = MonitorGenerator::default();
		catch_unwind(AssertUnwindSafe(|| generator.generate(false))).unwrap();
	}
}
