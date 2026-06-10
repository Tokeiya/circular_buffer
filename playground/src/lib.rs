// 1. nightly フィーチャーが有効なときだけ #![feature(test)] を有効化
#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(feature = "nightly")]
mod bench;

// 3. 常に有効（Stableでも動く共通コード）
#[path = "../../circular_buffer/tests/drop_observe/mod.rs"]
mod droppable;

pub use droppable::*;
