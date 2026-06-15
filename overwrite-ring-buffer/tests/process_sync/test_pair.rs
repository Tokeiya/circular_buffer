#![allow(dead_code)]

use crate::drop_observe::*;
use overwrite_ring_buffer::CircularBuffer;
use std::collections::HashMap;
use std::iter::{DoubleEndedIterator, ExactSizeIterator};

const THRESHOLD: usize = 1024;

fn assert_probe(actual: &Probe, expected: &Probe) {
	assert_eq!(actual.id(), expected.id());
	assert_eq!(actual.is_dropped(), expected.is_dropped());
}

pub struct TestPair<A, E>
where
	A: CircularBuffer<Probe>,
	E: CircularBuffer<Probe>,
{
	actual: A,
	expected: E,
	act_gen: MonitorGenerator,
	exp_gen: MonitorGenerator,
	act_hash: HashMap<usize, Monitor>,
	exp_hash: HashMap<usize, Monitor>,
}

impl<A, E> Drop for TestPair<A, E>
where
	A: CircularBuffer<Probe>,
	E: CircularBuffer<Probe>,
{
	fn drop(&mut self) {
		self.assert();
		dbg!("TestPair dropped successfully.");
	}
}

impl<A, E> TestPair<A, E>
where
	A: CircularBuffer<Probe>,
	E: CircularBuffer<Probe>,
{
	pub fn new(actual: A, expected: E) -> Self {
		Self {
			actual,
			expected,
			act_gen: MonitorGenerator::default(),
			exp_gen: MonitorGenerator::default(),
			act_hash: HashMap::default(),
			exp_hash: HashMap::default(),
		}
	}
	pub fn assert(&mut self) {
		let act = &self.actual;
		let exp = &self.expected;

		assert_eq!(act.capacity(), exp.capacity(), "capacity");
		assert_eq!(act.len(), exp.len(), "len");

		for (a, e) in act.iter().zip(exp.iter()) {
			assert_probe(a, e);
			assert!(!a.is_dropped());
			assert!(!e.is_dropped());
		}

		assert_eq!(self.act_hash.len(), self.exp_hash.len());
		assert!(self.act_hash.keys().all(|k| self.exp_hash.contains_key(k)));

		let mut expired = Vec::default();
		let enable = self.exp_hash.keys().copied().collect::<Vec<_>>();

		for key in enable.into_iter() {
			let a = self.act_hash.get(&key).unwrap();
			let e = self.exp_hash.get(&key).unwrap();

			assert_eq!(a.id(), key);
			assert_eq!(e.id(), key);
			assert_eq!(a.is_dropped(), e.is_dropped());

			if e.is_dropped() {
				expired.push(key);
			}
		}

		for key in expired {
			self.act_hash.remove(&key);
			self.exp_hash.remove(&key);
		}

		dbg!("Assertion passed.");
	}

	pub fn enqueue(&mut self) {
		let act = self.exp_gen.generate();
		let exp = self.act_gen.generate();
		assert_eq!(act.id(), exp.id());

		self.actual.enqueue(act.payout_probe());
		self.expected.enqueue(exp.payout_probe());

		self.act_hash.insert(act.id(), act);
		self.exp_hash.insert(exp.id(), exp);

		if self.act_hash.len() >= THRESHOLD {
			self.assert();
		}
	}

	pub fn strict_enqueue(&mut self) {
		self.enqueue();
		self.assert();
	}

	pub fn dequeue(&mut self) {
		let act = self.actual.dequeue();
		let exp = self.expected.dequeue();

		match (act.as_ref(), exp.as_ref()) {
			(Some(a), Some(e)) => assert_probe(a, e),
			(None, None) => return,
			_ => panic!(" dequeue error"),
		}

		let exp = exp.unwrap();
		let act = act.unwrap();

		let id = exp.id();

		drop(act);
		drop(exp);

		assert!(self.act_hash.get(&id).unwrap().is_dropped());
		assert!(self.exp_hash.get(&id).unwrap().is_dropped());

		self.act_hash.remove(&id).unwrap();
		self.exp_hash.remove(&id).unwrap();

		if self.act_hash.len() >= THRESHOLD {
			self.assert();
		}
	}

	pub fn strict_dequeue(&mut self) {
		self.dequeue();
		self.assert();
	}

	pub fn get_probe(&mut self) -> (Probe, Probe) {
		let act = self.act_gen.generate();
		let exp = self.exp_gen.generate();
		assert_eq!(act.id(), exp.id());

		let ret = (act.payout_probe(), exp.payout_probe());

		self.act_hash.insert(act.id(), act);
		self.exp_hash.insert(exp.id(), exp);

		ret
	}

	pub fn iter_mut(
		&mut self,
	) -> impl DoubleEndedIterator<Item = (&mut Probe, &mut Probe)> + ExactSizeIterator {
		if self.act_hash.len() >= THRESHOLD {
			self.assert();
		}

		self.actual.iter_mut().zip(self.expected.iter_mut())
	}

	pub fn iter(&self) -> impl ExactSizeIterator + DoubleEndedIterator<Item = (&Probe, &Probe)> {
		self.actual.iter().zip(self.expected.iter())
	}

	pub fn get(&self, index: usize) -> (&Probe, &Probe) {
		(&self.actual[index], &self.expected[index])
	}

	pub fn get_mut(&mut self, index: usize) -> (&mut Probe, &mut Probe) {
		if self.act_hash.len() >= THRESHOLD {
			self.assert();
		}

		(&mut self.actual[index], &mut self.expected[index])
	}

	pub fn len(&self) -> usize {
		assert_eq!(self.actual.len(), self.expected.len());
		self.expected.len()
	}

	pub fn clear(&mut self) {
		self.actual.clear();
		self.expected.clear();

		self.assert();
	}
}
