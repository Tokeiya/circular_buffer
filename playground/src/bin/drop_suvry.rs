struct Foo;
impl Drop for Foo {
	fn drop(&mut self) {
		println!("Foo Dropped");
	}
}

struct Bar(Foo);

impl Drop for Bar {
	fn drop(&mut self) {
		println!("Bar Dropped")
	}
}

struct Hoge(Bar);

impl Drop for Hoge {
	fn drop(&mut self) {}
}

fn main() {
	let foo = Bar(Foo);
	panic!("Scheduled")
}
