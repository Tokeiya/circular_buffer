#![allow(dead_code)]

#[cfg(feature = "nightly")]
extern crate test;

#[cfg(all(feature = "nightly", test))]
mod benchmark {
	use super::*;
	use overwrite_ring_buffer::CircularBuffer;
	use std::hint::black_box;
	use test::Bencher;

	use crate::control::Expected;
	use overwrite_ring_buffer::fixed::{
		Buffer as FixedBuffer, GeneralIndexCoordinator as FixedGeneral,
		Pow2IndexCoordinator as FixedPow2,
	};
	use overwrite_ring_buffer::resizable::{
		Buffer as ResizableBuffer, GeneralIndexCoordinator as ResizableGeneral,
		Pow2IndexCoordinator as ResizablePow2,
	};

	const POW2: usize = 1024;
	const GENERAL: usize = 1023;
	const ITERATIONS: usize = 1_000_000;
	const MASK: usize = 63;

	fn process<T: CircularBuffer<usize>>(mut buffer: T) {
		let mut accum = 0;

		for i in 0..ITERATIONS {
			buffer.enqueue(i);

			// if i & MASK == 0 {
			// 	for idx in 0..buffer.len() {
			// 		accum += buffer[idx];
			// 	}
			// }
		}

		accum = buffer.iter().sum::<usize>();

		black_box(accum);
	}

	#[bench]
	fn fixed_pow2(bencher: &mut Bencher) {
		bencher.iter(|| {
			let buffer = FixedBuffer::<usize, FixedPow2<POW2>, POW2>::default();
			process(buffer);
		});
	}

	#[bench]
	fn fixed_general(bencher: &mut Bencher) {
		bencher.iter(|| {
			let buffer = FixedBuffer::<usize, FixedGeneral<GENERAL>, GENERAL>::default();
			process(buffer);
		});
	}

	#[bench]
	fn resizable_pow2(bencher: &mut Bencher) {
		bencher.iter(|| {
			let buffer =
				ResizableBuffer::<usize, ResizablePow2>::new(ResizablePow2::try_new(POW2).unwrap());
			process(buffer);
		});
	}

	#[bench]
	fn resizable_general(bencher: &mut Bencher) {
		bencher.iter(|| {
			let buffer = ResizableBuffer::<usize, ResizableGeneral>::new(
				ResizableGeneral::try_new(GENERAL).unwrap(),
			);
			process(buffer);
		});
	}

	#[bench]
	fn control_general(bencher: &mut Bencher) {
		bencher.iter(|| {
			let buffer = Expected::<usize, GENERAL>::default();
			process(buffer);
		});
	}

	#[bench]
	fn control_pow2(bencher: &mut Bencher) {
		bencher.iter(|| {
			let buffer = Expected::<usize, POW2>::default();
			process(buffer);
		});
	}
}
