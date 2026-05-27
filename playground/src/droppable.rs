use std::sync::atomic::AtomicUsize;

pub struct Droppable(usize);

impl Default for Droppable {
	fn default() -> Self {
		static SEED: AtomicUsize = AtomicUsize::new(0);
		Self(SEED.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
	}
}

impl Droppable {
	pub fn identity(&self) -> usize {
		self.0
	}
}

impl Drop for Droppable {
	fn drop(&mut self) {
		println!("Droppable dropped with identity: {}", self.0);
	}
}
