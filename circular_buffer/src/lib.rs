mod circular_buffer;
mod error;
pub mod fixed;

#[cfg(test)]
#[path = "../tests/drop_observe/mod.rs"]
mod test_shared;

pub use circular_buffer::CircularBuffer;
pub use error::Error;
pub use error::Result;
