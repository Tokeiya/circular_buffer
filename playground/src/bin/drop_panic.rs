struct A;
struct B;

impl Drop for A {
	fn drop(&mut self) {
		println!("drop A");
		panic!("panic in A::drop");
	}
}

impl Drop for B {
	fn drop(&mut self) {
		println!("drop B");
		panic!("panic in B::drop");
	}
}

fn main() {
	let _a = A;
	let _b = B;
	
	panic!("first panic");
}