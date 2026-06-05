use thiserror::Error as ThisErr;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisErr, Debug)]
pub enum Error {
	#[error("index {index} is out of range for length {len}")]
	IndexOutOfRange { index: usize, len: usize },
	#[error("State is empty")]
	Empty,
	#[error("Capacity {0} is not a power of 2.")]
	CapacityNotPow2(usize),
	#[error("Capacity must be greater than zero.")]
	ZeroCapacity,
}
