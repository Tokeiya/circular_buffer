use crate::index_coordinator::IndexCoordinator;

pub trait ResizableIndexCoordinator: IndexCoordinator {
	fn empty_like(&self) -> Self;
}
