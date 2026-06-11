trait Base {
	fn foo(&self) -> usize;
}

trait Derived: Base {
	fn bar(&self) -> usize;
}

fn main() {}
