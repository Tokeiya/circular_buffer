mod drop_observe;
mod process_sync;

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

#[test]
fn enqueue_dequeue_iter_mut() {
	let (seed, mut rng) = gen_rnd();
	dbg!(seed);
	let mut pair = TestPair::<ActualFixture, ExpectedFixture>::default();
	pair.assert();

	let proc = [Process::Enqueue, Process::Dequeue, Process::IterMut];

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
				let value: Vec<Option<(Probe, Probe)>> = (0..pair.len())
					.map(|_| {
						if rng.random_bool(0.5) {
							Some(pair.get_probe())
						} else {
							None
						}
					})
					.collect();

				let iter_mut = pair.iter_mut();

				for ((a, e), p) in iter_mut.zip(value.into_iter()) {
					if let Some((ap, ep)) = p {
						*a = ap;
						*e = ep;
					}
				}
			}
			_ => unreachable!(),
		}
	}
}
