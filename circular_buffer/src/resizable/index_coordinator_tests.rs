#![cfg(test)]
use super::resizable_index_coordinator::ResizableIndexCoordinator;
pub(super) trait TestExtension: ResizableIndexCoordinator {
	fn fixture() -> Self;
}
