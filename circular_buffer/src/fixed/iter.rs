pub struct Iter<'a, T, const N: usize> {
	storage: &'a [T; N],
	idx: usize,
	len: usize,
	head: usize,
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}
