fn main() {
	let mut vec = vec![1, 2, 3, 4, 5];
	
	let elem={
		let mut iter=vec.iter_mut();
		iter.next().unwrap()
	};
	
	
	
	*elem=20;
	
}