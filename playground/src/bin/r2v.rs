const BASE: usize = 8;
const HEAD: usize = 3;

fn real_to_virtual(idx: usize) -> usize {
	if idx >= BASE {
		panic!("index out of range");
	}

	let tmp = idx.wrapping_sub(HEAD);
	tmp & BASE - 1
}

fn virtual_to_real(idx: usize) -> usize {
	if idx >= BASE {
		panic!("index out of range");
	}

	let tmp = idx.wrapping_add(HEAD);
	tmp & BASE - 1
}

fn main() {
	for i in (0..BASE).map(|i| (i + HEAD) & BASE - 1) {
		println!("real_to_virtual({})={}", i, real_to_virtual(i));
	}

	println!("------------------------");

	for i in 0..BASE {
		println!("virtual_to_real({})={}", i, virtual_to_real(i));
	}

	let a = real_to_virtual(BASE);
}
