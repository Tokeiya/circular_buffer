// 1. nightly フィーチャーが有効なときだけ #![feature(test)] を有効化
#![cfg_attr(feature = "nightly", feature(test))]

// 2. nightly フィーチャーが有効なときだけコンパイルに含めるモジュール
#[cfg(feature = "nightly")]
pub mod droppable;

#[cfg(feature = "nightly")]
mod bench;

// 3. 常に有効（Stableでも動く共通コード）
#[path = "../../circular_buffer/tests/drop_observe/mod.rs"]
mod shared;

pub use shared::*;