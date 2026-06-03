use crate::IndexCoordinator;

pub trait FixedIndexCoordinator<const N: usize>: IndexCoordinator {}
