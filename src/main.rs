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
mod edit_member;
pub(crate) mod sprint;
pub(crate) mod render;

const OWNER: u64 = 5000;

fn main() -> Result<(), Box<dyn Error>> {
	let data_dir = {
		let mut dir = dirs::home_dir().expect("Home exists");
		dir.push(format!(".{}", APP_NAME));
		dir
	};
	let chad = Chad::connect(&data_dir);
	let spark = pick_squad::Spark { chad };
	app::run(spark, None)?;
	Ok(())
}

enum YardId {
	MemberSymbolEdit,
	MemberPriceEdit,
	SquadMembersList,
	EditSquadList,
	NameField,
	PickSquadList,
}

impl YardId {
	pub fn as_i32(&self) -> i32 {
		match self {
			YardId::MemberSymbolEdit => 701,
			YardId::MemberPriceEdit => 702,
			YardId::SquadMembersList => 703,
			YardId::EditSquadList => 704,
			YardId::NameField => 705,
			YardId::PickSquadList => 706,
		}
	}
}

#[cfg(debug_assertions)]
const APP_NAME: &str = "chad-debug";
#[cfg(not(debug_assertions))]
const APP_NAME: &str = "chad";
