#[derive(Debug)]
pub enum Sample {
	Integer(i64),
	Text(String),
}

fn main() {
	let mut s = Sample::Integer(10);
	println!("{:?}", s);
	change(&mut s);
	println!("{:?}", s);
}

fn change(sample: &mut Sample) {
	*sample = match sample {
		Sample::Integer(_) => Sample::Text("text".to_string()),
		Sample::Text(_) => Sample::Integer(42),
	};
}
