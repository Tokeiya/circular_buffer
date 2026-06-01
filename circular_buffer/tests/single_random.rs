mod drop_observe;
mod process_sync;

use circular_buffer::CircularBuffer;
use circular_buffer::fixed::Buffer;
use drop_observe::*;
use process_sync::Expected;
use process_sync::test_pair::TestPair;
use rand::RngExt;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

const ITERATION: usize = 1024;
const SIZE: usize = 64;
type ActualFixture = Buffer<Probe, SIZE>;
type ExpectedFixture = Expected<Probe, SIZE>;

#[derive(Eq, PartialEq)]
enum Process {
	Enqueue,
	Dequeue,
	IndexMut,
	IterMut,
}

fn gen_rnd() -> (u64, ChaCha8Rng) {
	let seed = rand::rng().next_u64();
	let rng = ChaCha8Rng::seed_from_u64(seed);
	(seed, rng)
}

#[test]
fn enqueue_dequeue() {
	let (seed, mut rng) = gen_rnd();
	dbg!(seed);
	let mut pair = TestPair::<ActualFixture, ExpectedFixture>::default();
	pair.assert();

	let proc = [Process::Enqueue, Process::Dequeue];

	for _ in 0..ITERATION {
		match proc.choose(&mut rng).unwrap() {
			Process::Enqueue => {
				for _ in 0..rng.random_range(0..=SIZE) {
					pair.enqueue()
				}
			}
			Process::Dequeue => {
				for _ in 0..rng.random_range(0..=SIZE) {
					pair.dequeue()
				}
			}
			_ => unreachable!(),
		}
	}

	pair.assert();
}

fn iter_mut_process<A: CircularBuffer<Probe>, E: CircularBuffer<Probe>>(
	pair: &mut TestPair<A, E>,
	rng: &mut impl Rng,
) {
	let value: Vec<Option<(Probe, Probe)>> = (0..pair.len())
		.map(|_| {
			if rng.random_bool(0.5) {
				Some(pair.get_probe())
			} else {
				None
			}
		})
		.collect();

	let mut iter_mut = pair.iter_mut().zip(value);

	loop {
		if rng.random_bool(0.5) {
			if let Some(((a, e), val)) = iter_mut.next() {
				if let Some((ap, ep)) = val {
					*a = ap;
					*e = ep;
				}
			} else {
				break;
			}
		} else {
			if let Some(((a, e), val)) = iter_mut.next_back() {
				if let Some((ap, ep)) = val {
					*a = ap;
					*e = ep;
				}
			} else {
				break;
			}
		}
	}
}

fn index_mut_process<A: CircularBuffer<Probe>, E: CircularBuffer<Probe>>(
	pair: &mut TestPair<A, E>,
	rng: &mut impl Rng,
) {
	let mut value: Vec<Option<(Probe, Probe)>> = (0..pair.len())
		.map(|_| {
			if rng.random_bool(0.5) {
				Some(pair.get_probe())
			} else {
				None
			}
		})
		.collect();

	#[allow(clippy::needless_range_loop)]
	for idx in 0..value.len() {
		if value[idx].is_some() {
			let (ap, ep) = value[idx].take().unwrap();
			let (a, e) = pair.get_mut(idx);
			*a = ap;
			*e = ep;
		}
	}
}

#[test]
fn all_process() {
	let (seed, mut rng) = gen_rnd();
	dbg!(seed);
	let mut pair = TestPair::<ActualFixture, ExpectedFixture>::default();
	pair.assert();

	let proc = [
		Process::Enqueue,
		Process::Dequeue,
		Process::IterMut,
		Process::IndexMut,
	];

	for _ in 0..ITERATION {
		match proc.choose(&mut rng).unwrap() {
			Process::Enqueue => {
				for _ in 0..rng.random_range(0..=SIZE) {
					pair.enqueue()
				}
			}
			Process::Dequeue => {
				for _ in 0..rng.random_range(0..=SIZE) {
					pair.dequeue()
				}
			}
			Process::IterMut => {
				iter_mut_process(&mut pair, &mut rng);
			}
			Process::IndexMut => {
				index_mut_process(&mut pair, &mut rng);
			}
			_ => unreachable!(),
		}
	}
}
