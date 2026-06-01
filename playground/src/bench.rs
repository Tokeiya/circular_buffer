#![feature(test)]

extern crate test;

const BASE: usize = 64;
const MASK: usize = BASE - 1;
pub fn use_mod(x: usize) -> usize {
	x & MASK
}

pub fn use_ops(x: usize) -> usize {
	x % MASK
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::hint::black_box;
	use test::Bencher;

	const ITER: usize = 10_000;

	#[bench]
	fn bench_use_mod(b: &mut Bencher) {
		b.iter(|| {
			for i in 0..ITER {
				_ = black_box(use_mod(i))
			}
		});
	}

	#[bench]
	fn bench_use_ops(b: &mut Bencher) {
		b.iter(|| {
			for i in 0..ITER {
				_ = black_box(use_ops(i))
			}
		});
	}
}
