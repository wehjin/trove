extern crate chad_core;
extern crate dirs;
extern crate echo_lib;
extern crate rand;
extern crate stringedit;
extern crate yui;

use std::error::Error;

use chad_core::chad::Chad;
use yui::app;

mod edit_squad;
mod pick_squad;
pub(crate) mod sprint;

const OWNER: u64 = 5000;

fn main() -> Result<(), Box<dyn Error>> {
	let chad = Chad::connect_tmp();
	let spark = pick_squad::Spark { chad };
	app::run(spark, None)?;
	Ok(())
}

enum YardId {
	EditSquadList,
	NameField,
	PickSquadList,
}

impl YardId {
	pub fn as_i32(&self) -> i32 {
		match self {
			YardId::EditSquadList => 704,
			YardId::NameField => 705,
			YardId::PickSquadList => 706,
		}
	}
}