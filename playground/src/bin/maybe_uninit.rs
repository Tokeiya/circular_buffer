use playground::droppable::Droppable;
use std::mem::MaybeUninit;
fn main() {
	let mut fixture = MaybeUninit::<Droppable>::zeroed();

	//Legal
	{
		fixture.write(Droppable::default());
	}

	legal_replace(&mut fixture);

	println!("Done");

	//finally
	{
		unsafe { fixture.assume_init_drop() };
	}
}

fn legal_replace(item: &mut MaybeUninit<Droppable>) {
	let r = unsafe { item.assume_init_mut() };
	*r = Droppable::default();
}

fn illegal_replace(item: &mut MaybeUninit<Droppable>) {
	item.write(Droppable::default());
}

fn illegal_to() {
	let mut fixture = MaybeUninit::<Droppable>::zeroed();
	unsafe { fixture.assume_init_drop() };
}

fn danger() {
	let mut fix = MaybeUninit::<Droppable>::uninit();
	*unsafe { fix.assume_init_mut() } = Droppable::default();
}
