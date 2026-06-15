use crate::drop_observe::Probe;
use crate::process_sync::test_pair::TestPair;
use overwrite_ring_buffer::CircularBuffer;
use rand::prelude::IndexedRandom;
use rand::{Rng, RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

const ITERATION: usize = 8192;
#[derive(Eq, PartialEq)]
enum Process {
	Enqueue,
	Dequeue,
	IndexMut,
	IterMut,
	Index,
	Iter,
	Clear,
}

fn gen_rnd() -> (u64, ChaCha8Rng) {
	let seed = rand::rng().next_u64();
	let rng = ChaCha8Rng::seed_from_u64(seed);
	(seed, rng)
}

pub fn enqueue_dequeue_impl<A: CircularBuffer<Probe>, E: CircularBuffer<Probe>, const N: usize>(
	actual: A,
	expected: E,
) {
	let (seed, mut rng) = gen_rnd();
	dbg!(seed);
	let mut pair = TestPair::<A, E>::new(actual, expected);

	pair.assert();

	let proc = [Process::Enqueue, Process::Dequeue];

	for _ in 0..ITERATION {
		match proc.choose(&mut rng).unwrap() {
			Process::Enqueue => {
				for _ in 0..rng.random_range(0..=N) {
					pair.enqueue()
				}
			}
			Process::Dequeue => {
				for _ in 0..rng.random_range(0..=N) {
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

fn index_process<A: CircularBuffer<Probe>, E: CircularBuffer<Probe>>(pair: &mut TestPair<A, E>) {
	for idx in 0..pair.len() {
		let (a, e) = pair.get(idx);
		assert_eq!(a.id(), e.id());
	}
}

fn iter_process<A: CircularBuffer<Probe>, E: CircularBuffer<Probe>>(pair: &mut TestPair<A, E>) {
	for (a, e) in pair.iter() {
		assert_eq!(a.id(), e.id());
	}
}

fn clear_process<A: CircularBuffer<Probe>, E: CircularBuffer<Probe>>(pair: &mut TestPair<A, E>) {
	pair.clear()
}

pub fn all_process_impl<A: CircularBuffer<Probe>, E: CircularBuffer<Probe>, const N: usize>(
	actual: A,
	expected: E,
) {
	let (seed, mut rng) = gen_rnd();
	dbg!(seed);
	let mut pair = TestPair::<A, E>::new(actual, expected);
	pair.assert();

	let proc = [
		Process::Enqueue,
		Process::Dequeue,
		Process::IterMut,
		Process::IndexMut,
		Process::Index,
		Process::Iter,
		Process::Clear,
	];

	#[allow(unreachable_patterns)]
	for _ in 0..ITERATION {
		match proc.choose(&mut rng).unwrap() {
			Process::Enqueue => {
				for _ in 0..rng.random_range(0..=N) {
					pair.enqueue()
				}
			}
			Process::Dequeue => {
				for _ in 0..rng.random_range(0..=N) {
					pair.dequeue()
				}
			}
			Process::IterMut => iter_mut_process(&mut pair, &mut rng),
			Process::IndexMut => index_mut_process(&mut pair, &mut rng),
			Process::Index => index_process(&mut pair),
			Process::Iter => iter_process(&mut pair),
			Process::Clear => clear_process(&mut pair),
			_ => unreachable!(),
		}
	}
}
