#![allow(dead_code)]

// 1. nightly フィーチャーがあるときだけ、Nightly専用の標準テストクレートを読み込む
#[cfg(feature = "nightly")]
extern crate test;

fn common_mod<const N: usize>(value: usize) -> usize {
	value % N
}

fn pow2_mod<const N: usize>(value: usize) -> usize {
	value & (N - 1)
}

trait Modulo<const N: usize> {
	fn modulo(&self, value: usize) -> usize;
}

struct Dynamic<const N: usize>(Box<dyn Modulo<N>>);

impl<const N: usize> Modulo<N> for Dynamic<N> {
	fn modulo(&self, value: usize) -> usize {
		self.0.modulo(value)
	}
}

enum Static<const N: usize> {
	Common,
	Pow2,
}

impl<const N: usize> Modulo<N> for Static<N> {
	fn modulo(&self, value: usize) -> usize {
		match self {
			Static::Common => common_mod::<N>(value),
			Static::Pow2 => pow2_mod::<N>(value),
		}
	}
}

// 2. このモジュール全体を「nightly フィーチャーが有効なとき」かつ「テスト/ベンチマーク実行時」のみに限定する
#[cfg(all(feature = "nightly", test))]
mod benchmark {
	use super::*;
	use std::hint::black_box;
	use test::Bencher; // 👈 ここで上記の extern crate test を利用しているため、隔離が必要
	
	const ITERATIONS: usize = 100_000;
	
	#[bench]
	fn dynamic_normal(bencher: &mut Bencher) {
		bencher.iter(|| {
			let mut accum = 0usize;
			let fixture = Dynamic::<100>(Box::new(Static::Common));
			for i in 0..ITERATIONS {
				accum += fixture.modulo(i);
			}
			
			black_box(accum);
		});
	}
	
	#[bench]
	fn static_normal(bencher: &mut Bencher) {
		bencher.iter(|| {
			let mut accum = 0usize;
			let fixture = Static::<100>::Common;
			for i in 0..ITERATIONS {
				accum += fixture.modulo(i);
			}
			black_box(accum);
		});
	}
	
	#[bench]
	fn dynamic_pow2(bencher: &mut Bencher) {
		bencher.iter(|| {
			let mut accum = 0usize;
			let fixture = Dynamic::<256>(Box::new(Static::Pow2));
			for i in 0..ITERATIONS {
				accum += fixture.modulo(i);
			}
			
			black_box(accum);
		});
	}
	
	#[bench]
	fn static_pow2(bencher: &mut Bencher) {
		bencher.iter(|| {
			let mut accum = 0usize;
			let fixture = Static::<256>::Pow2;
			for i in 0..ITERATIONS {
				accum += fixture.modulo(i);
			}
			
			black_box(accum);
		});
	}
}