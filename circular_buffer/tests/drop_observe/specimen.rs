use super::probe::Probe;

#[derive(Debug)]
pub struct Specimen(Probe);

impl Specimen {
	pub(super) fn new(probe: Probe) -> Self {
		assert!(!probe.is_dropped());
		Self(probe)
	}

	pub fn id(&self) -> usize {
		self.0.id()
	}

	pub fn is_dropped(&self) -> bool {
		self.0.is_dropped()
	}
}

impl Drop for Specimen {
	fn drop(&mut self) {
		self.0.mark_dropped();
	}
}
