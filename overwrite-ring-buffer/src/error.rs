use thiserror::Error as ThisErr;

/// A specialized [`Result`] type used by this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur while operating on a circular buffer or its index
/// coordinator.
#[derive(ThisErr, Debug)]
pub enum Error {
	/// The specified logical index is outside the range of initialized elements.
	///
	/// Valid indices are in the range `0..len`.
	#[error("index {index} is out of range for length {len}")]
	IndexOutOfRange { index: usize, len: usize },

	/// The requested operation cannot be performed because the state is empty.
	#[error("State is empty")]
	Empty,

	/// The specified capacity is not a power of two.
	///
	/// This error is returned by implementations that require power-of-two
	/// capacities for bitmask-based index calculation.
	#[error("Capacity {0} is not a power of 2.")]
	CapacityNotPow2(usize),

	/// The specified capacity is zero.
	///
	/// Circular buffers must have a capacity greater than zero.
	#[error("Capacity must be greater than zero.")]
	ZeroCapacity,
}
