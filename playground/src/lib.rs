#![feature(test)]
pub mod droppable;

mod bench;
#[path = "../../circular_buffer/tests/drop_observe/mod.rs"]
mod shared;

pub use shared::*;
