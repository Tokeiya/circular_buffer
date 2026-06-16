/// Internal support for the sealed trait pattern.
pub(crate) mod sealed {
	/// Prevents external crates from implementing selected public traits.
	pub trait Sealed {}
}

/// Coordinates logical buffer positions with physical storage indices.
///
/// `IndexCoordinator` is responsible for tracking the logical state of a
/// circular buffer, such as its head position, tail position, length, and
/// capacity.
///
/// This trait does not own or access the stored elements themselves. Instead,
/// it only manages indices into the backing storage.
///
/// # Logical and physical indices
///
/// A logical index is an index as seen by users of the buffer. For example,
/// logical index `0` refers to the oldest element currently stored in the
/// buffer.
///
/// A physical index is an index into the actual backing storage.
///
/// Implementations of this trait translate logical indices into physical
/// storage indices while preserving circular-buffer semantics.
///
/// # Sealed trait
///
/// This trait is sealed and cannot be implemented outside this crate.
pub trait IndexCoordinator: Clone + sealed::Sealed {
	/// Returns the physical index of the logical front element.
	///
	/// The head is the oldest element currently stored in the buffer.
	///
	/// # Errors
	///
	/// Returns an error if the coordinator is empty.
	fn head_index(&self) -> crate::Result<usize>;

	/// Returns the physical index at which the next element should be written.
	///
	/// If the buffer is full, this index may refer to the slot containing the
	/// current head element, which will be overwritten by an enqueue operation.
	fn tail_index(&self) -> crate::Result<usize>;

	/// Advances the coordinator after an enqueue operation.
	///
	/// If the buffer was not full, this increases the length by one.
	///
	/// If the buffer was full, this advances both the head and tail positions,
	/// preserving the capacity while logically discarding the oldest element.
	fn enqueue_index(&mut self);

	/// Advances the coordinator after removing the front element.
	///
	/// This removes the logical head element from the coordinator state.
	///
	/// # Errors
	///
	/// Returns an error if the coordinator is empty.
	fn dequeue_index(&mut self) -> crate::Result<()>;

	/// Updates the coordinator after removing the back element.
	///
	/// This removes the newest element from the coordinator state.
	///
	/// # Errors
	///
	/// Returns an error if the coordinator is empty.
	fn pop_index(&mut self) -> crate::Result<()>;

	/// Resolves a logical index into a physical storage index.
	///
	/// Logical index `0` refers to the oldest element currently stored in the
	/// buffer. The valid logical index range is `0..self.len()`.
	///
	/// # Errors
	///
	/// Returns an error if `idx` is outside the valid logical index range.
	fn resolve_index(&self, idx: usize) -> crate::Result<usize>;

	/// Returns the maximum number of elements that can be represented.
	fn capacity(&self) -> usize;

	/// Returns the number of elements currently represented.
	///
	/// The returned value is always in the range `0..=self.capacity()`.
	fn len(&self) -> usize;

	/// Returns `true` if the coordinator currently represents no elements.
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Returns `true` if the coordinator currently represents exactly
	/// [`capacity`](Self::capacity) elements.
	fn is_full(&self) -> bool {
		self.len() == self.capacity()
	}
}
