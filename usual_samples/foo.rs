fn main() {
	let mut integer = 42i32;
	let mut_ref = &mut integer;
	*mut_ref += 1;

	let mut string = String::from("Hello");
	let mut_ref = &mut string;
	mut_ref.push_str(", world!");

	*mut_ref = String::from("hogemoge");
}
