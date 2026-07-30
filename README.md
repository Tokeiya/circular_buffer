# overwrite-ring-buffer

[![Crates.io](https://img.shields.io/crates/v/overwrite-ring-buffer.svg)](https://crates.io/crates/overwrite-ring-buffer)
[![Docs.rs](https://docs.rs/overwrite-ring-buffer/badge.svg)](https://docs.rs/overwrite-ring-buffer)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A fixed-size and resizable overwrite ring buffer for Rust.

`overwrite-ring-buffer` provides circular buffers that keep the most recent values.
When the buffer is full, inserting a new item overwrites the oldest item.

This crate is useful when you want bounded storage for recent data, logs, samples,
events, measurements, or other stream-like values where keeping the latest items is
more important than preserving all historical values.

## Features

* Fixed-capacity ring buffer using const generics
* Runtime-sized ring buffer
* Overwrite-on-full behavior
* Index access where `buffer[0]` is the oldest element
* Immutable and mutable iteration
* Power-of-two optimized index coordinators
* General-purpose coordinators for arbitrary capacities
* Creation of an empty buffer with matching configuration through `empty_like`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
overwrite-ring-buffer = "0.19.2"
```

Then import it in Rust as:

```rust
use overwrite_ring_buffer::*;
```

## Basic usage

```rust
use overwrite_ring_buffer::{
	CircularBuffer,
	fixed::{Buffer, Pow2IndexCoordinator},
};

const CAPACITY: usize = 4;

fn main() {
	let mut buffer: Buffer<i32, Pow2IndexCoordinator<CAPACITY>, CAPACITY> = Buffer::default();
	
	buffer.enqueue(10);
	buffer.enqueue(20);
	buffer.enqueue(30);
	buffer.enqueue(40);
	
	assert_eq!(buffer.len(), 4);
	assert_eq!(buffer.capacity(), 4);
	
	assert_eq!(buffer[0], 10);
	assert_eq!(buffer[1], 20);
	assert_eq!(buffer[2], 30);
	assert_eq!(buffer[3], 40);
	
	// The buffer is full, so this overwrites the oldest value: 10.
	buffer.enqueue(50);
	
	assert_eq!(buffer.len(), 4);
	assert_eq!(buffer[0], 20);
	assert_eq!(buffer[1], 30);
	assert_eq!(buffer[2], 40);
	assert_eq!(buffer[3], 50);
}
```

## Dequeue

`dequeue` removes and returns the oldest item.

```rust
use overwrite_ring_buffer::{
	CircularBuffer,
	fixed::{Buffer, Pow2IndexCoordinator},
};

const CAPACITY: usize = 4;

fn main() {
	let mut buffer: Buffer<&str, Pow2IndexCoordinator<CAPACITY>, CAPACITY> = Buffer::default();
	
	buffer.enqueue("a");
	buffer.enqueue("b");
	buffer.enqueue("c");
	
	assert_eq!(buffer.dequeue(), Some("a"));
	assert_eq!(buffer.dequeue(), Some("b"));
	assert_eq!(buffer.dequeue(), Some("c"));
	assert_eq!(buffer.dequeue(), None);
}
```

## Iteration

Items are iterated from the oldest to the newest.

```rust
use overwrite_ring_buffer::{
	CircularBuffer,
	fixed::{Buffer, Pow2IndexCoordinator},
};

const CAPACITY: usize = 4;

fn main() {
	let mut buffer: Buffer<i32, Pow2IndexCoordinator<CAPACITY>, CAPACITY> = Buffer::default();
	
	buffer.enqueue(1);
	buffer.enqueue(2);
	buffer.enqueue(3);
	buffer.enqueue(4);
	buffer.enqueue(5);
	
	let values: Vec<_> = buffer.iter().copied().collect();
	
	assert_eq!(values, vec![2, 3, 4, 5]);
}
```

## Mutable iteration

```rust
use overwrite_ring_buffer::{
	CircularBuffer,
	fixed::{Buffer, Pow2IndexCoordinator},
};

const CAPACITY: usize = 4;

fn main() {
	let mut buffer: Buffer<i32, Pow2IndexCoordinator<CAPACITY>, CAPACITY> = Buffer::default();
	
	buffer.enqueue(1);
	buffer.enqueue(2);
	buffer.enqueue(3);
	
	for value in buffer.iter_mut() {
		*value *= 10;
	}
	
	assert_eq!(buffer.iter().copied().collect::<Vec<_>>(), vec![10, 20, 30]);
}
```

## Resizable buffer

Use `resizable::Buffer` when the capacity is known at runtime.

```rust
use overwrite_ring_buffer::{
	CircularBuffer,
	resizable::{Buffer, Pow2IndexCoordinator},
};

fn main() -> overwrite_ring_buffer::Result<()> {
	let coordinator = Pow2IndexCoordinator::try_new(4)?;
	let mut buffer = Buffer::new(coordinator);
	
	buffer.enqueue(10);
	buffer.enqueue(20);
	buffer.enqueue(30);
	buffer.enqueue(40);
	buffer.enqueue(50);
	
	assert_eq!(buffer.iter().copied().collect::<Vec<_>>(), vec![20, 30, 40, 50]);
	
	Ok(())
}
```

## Creating an empty buffer with the same configuration

`empty_like` creates a new, empty buffer with the same concrete buffer type,
capacity, and index coordinator configuration as the source buffer. It does not
clone or move the stored elements, and the source buffer is left unchanged.

```rust
use overwrite_ring_buffer::{
	CircularBuffer,
	resizable::{Buffer, CoordinatorSelector},
};

fn main() -> overwrite_ring_buffer::Result<()> {
	let coordinator = CoordinatorSelector::new(6)?;
	let mut buffer = Buffer::new(coordinator);
	
	buffer.enqueue(10);
	buffer.enqueue(20);
	
	let mut empty = buffer.empty_like();
	
	assert_eq!(empty.capacity(), buffer.capacity());
	assert!(empty.is_empty());
	assert_eq!(buffer.iter().copied().collect::<Vec<_>>(), vec![10, 20]);
	
	empty.enqueue(99);
	assert_eq!(empty.dequeue(), Some(99));
	assert_eq!(buffer.iter().copied().collect::<Vec<_>>(), vec![10, 20]);
	
	Ok(())
}
```

## Choosing an index coordinator

This crate separates buffer storage from index coordination.

For fixed-size buffers:

```rust
use overwrite_ring_buffer::fixed::{
	Buffer,
	GeneralIndexCoordinator,
	Pow2IndexCoordinator,
};

// Optimized for power-of-two capacities.
type FastBuffer<T, const N: usize> = Buffer<T, Pow2IndexCoordinator<N>, N>;

// Works with arbitrary non-zero capacities.
type GeneralBuffer<T, const N: usize> = Buffer<T, GeneralIndexCoordinator<N>, N>;
```

For runtime-sized buffers:

```rust
use overwrite_ring_buffer::resizable::{
	Buffer,
	GeneralIndexCoordinator,
	Pow2IndexCoordinator,
};

fn main() -> overwrite_ring_buffer::Result<()> {
	let pow2 = Buffer::<i32, _>::new(Pow2IndexCoordinator::try_new(8)?);
	let general = Buffer::<i32, _>::new(GeneralIndexCoordinator::try_new(10)?);
	
	assert_eq!(pow2.capacity(), 8);
	assert_eq!(general.capacity(), 10);
	
	Ok(())
}
```

## Behavior

The logical order of the buffer is always oldest to newest.

```text
capacity = 3

enqueue(1)  -> [1]
enqueue(2)  -> [1, 2]
enqueue(3)  -> [1, 2, 3]
enqueue(4)  -> [2, 3, 4]
enqueue(5)  -> [3, 4, 5]
```

Indexing follows this logical order:

```rust
buffer[0] // oldest item
buffer[buffer.len() - 1] // newest item
```

Out-of-range indexing panics, matching Rust's standard indexing behavior.

## API overview

The central trait is `CircularBuffer<T>`:

```rust
pub trait CircularBuffer<T> {
	fn capacity(&self) -> usize;
	fn enqueue(&mut self, item: T);
	fn dequeue(&mut self) -> Option<T>;
	fn iter(&self) -> Self::Iter<'_>;
	fn iter_mut(&mut self) -> Self::MutIter<'_>;
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool;
	fn empty_like(&self) -> Self;
	fn clear(&mut self);
}
```

## Notes

This crate is intended for bounded, single-owner buffer use cases.
It is not a lock-free queue and does not provide built-in synchronization. If you
need shared access across threads, wrap the buffer in an appropriate synchronization
primitive such as `Mutex` or `RwLock`.

## License

This project is licensed under the MIT License.