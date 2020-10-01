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
mod add_lot;
pub(crate) mod sprint;
pub(crate) mod render;
pub(crate) mod compute;

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
	LotAccountEdit,
	LotSharesEdit,
	MemberSymbolEdit,
	MemberPriceEdit,
	SquadMembersList,
	EditSquadList,
	NameField,
	PickSquadList,
	MemberLotList,
}

impl YardId {
	pub fn as_i32(&self) -> i32 {
		match self {
			YardId::LotAccountEdit => 701,
			YardId::LotSharesEdit => 702,
			YardId::MemberSymbolEdit => 703,
			YardId::MemberPriceEdit => 704,
			YardId::SquadMembersList => 705,
			YardId::EditSquadList => 706,
			YardId::NameField => 707,
			YardId::PickSquadList => 708,
			YardId::MemberLotList => 709,
		}
	}
}

#[cfg(debug_assertions)]
const APP_NAME: &str = "chad-debug";
#[cfg(not(debug_assertions))]
const APP_NAME: &str = "chad";
