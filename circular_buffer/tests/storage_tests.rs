mod drop_observe;

use drop_observe::{Item, Monitor, MonitorGenerator};

#[test]
fn hoge() {
	let mut generator = MonitorGenerator::default();
	let p = generator.generate();
	let a = p.payout_probe();
}
