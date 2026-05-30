mod drop_observe;
mod process_sync;

use circular_buffer::CircularBuffer;
use circular_buffer::fixed::*;
use drop_observe::{Monitor, MonitorGenerator, Probe};
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::collections::{HashMap, VecDeque};
use std::env::args;
use std::mem::drop as consume;

#[derive(Copy, Clone)]
pub enum Process {
	Enqueue,
	Dequeue,
	Index,
}

const ITERATIONS: usize = 1_000;
type Fixture = Buffer<Probe, 1024>;
type Expected = VecDeque<Probe>;

#[test]
fn random_proc() {
	dbg!(std::env::current_dir().unwrap());
}
