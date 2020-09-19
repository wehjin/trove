extern crate dirs;
extern crate echo_lib;
extern crate rand;
extern crate stringedit;
extern crate yui;

use std::error::Error;

use yui::app;

use crate::main_page::MainPage;

mod data;
mod edit_lot;
mod list_assets;
mod view_asset;
mod list_factions;
mod main_page;

fn main() -> Result<(), Box<dyn Error>> {
	let echo = data::echo(".chad")?;
	app::run(MainPage::new(echo), None)?;
	Ok(())
}

enum YardId {
	AssetsTab,
	FactionsTab,
	FactionsList,
}

impl YardId {
	pub fn as_i32(&self) -> i32 {
		match self {
			YardId::AssetsTab => 700,
			YardId::FactionsTab => 701,
			YardId::FactionsList => 702,
		}
	}
}