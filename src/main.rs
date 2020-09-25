extern crate chad_core;
extern crate dirs;
extern crate echo_lib;
extern crate rand;
extern crate stringedit;
extern crate yui;

use std::error::Error;

use chad_core::chad::Chad;
use yui::app;

use crate::pick_squad::PickSquadSpark;

mod data;
mod edit_squad;
mod edit_lot;
mod list_assets;
mod view_asset;
mod list_factions;
mod main_page;
mod pick_squad;

const OWNER: u64 = 5000;

fn main() -> Result<(), Box<dyn Error>> {
	let chad = Chad::connect_tmp();
	let pick_squad = PickSquadSpark { chad };
	app::run(pick_squad, None)?;
	Ok(())
}

enum YardId {
	AssetsTab,
	FactionsTab,
	FactionsList,
	AssetList,
	EditSquadList,
	NameField,
	PickSquadList,
}

impl YardId {
	pub fn as_i32(&self) -> i32 {
		match self {
			YardId::AssetsTab => 700,
			YardId::FactionsTab => 701,
			YardId::FactionsList => 702,
			YardId::AssetList => 703,
			YardId::EditSquadList => 704,
			YardId::NameField => 705,
			YardId::PickSquadList => 706,
		}
	}
}