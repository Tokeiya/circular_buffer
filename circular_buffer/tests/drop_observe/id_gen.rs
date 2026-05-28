pub struct IdGenerator(usize);

impl IdGenerator {
	pub fn new(initial: usize) -> Self {
		Self(initial)
	}

	pub fn generate(&mut self) -> usize {
		let id = self.0;
		self.0 += 1;
		id
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn new() {
		for i in 0..10 {
			let fixture = IdGenerator::new(i);
			assert_eq!(fixture.0, i);
		}
	}

	#[test]
	fn generate() {
		let mut fixture = IdGenerator::new(0);
		for i in 0..10 {
			let id = fixture.generate();
			assert_eq!(id, i);
		}
	}
}
