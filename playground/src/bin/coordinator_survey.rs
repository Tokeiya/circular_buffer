use overwrite_ring_buffer::fixed::Pow2IndexCoordinator;
use overwrite_ring_buffer::IndexCoordinator;

fn main() {
	let mut coordinator = Pow2IndexCoordinator::<16>::default();
	for _ in 0..100 {
		coordinator.enqueue_index();
	}
	println!(
		"[0]:{},head:{} tail:{} len:{}",
		coordinator.resolve_index(0).unwrap(),
		coordinator.head_index().unwrap(),
		coordinator.tail_index().unwrap(),
		coordinator.len()
	);

	while coordinator.dequeue_index().is_ok() {
		if coordinator.len() != 0 {
			println!(
				"[0]:{},head:{} tail:{} len:{}",
				coordinator.resolve_index(0).unwrap(),
				coordinator.head_index().unwrap(),
				coordinator.tail_index().unwrap(),
				coordinator.len()
			);
		}
	}
}
