extern crate chad_core;
extern crate dirs;
extern crate echo_lib;
extern crate rand;
extern crate stringedit;
extern crate yui;

use std::error::Error;

pub use chad_core::Link as ChadLink;
use yui::app;

use crate::main_page::MainPage;

mod data;
mod edit_lot;
mod list_assets;
mod view_asset;
mod list_factions;
mod main_page;

fn main() -> Result<(), Box<dyn Error>> {
	let chad_link = {
		let mut data_folder = dirs::home_dir().expect("Home dir");
		data_folder.push(".chad");
		chad_core::connect(&data_folder)
	};
	let echo = data::echo(".chad")?;
	let main_page = MainPage::new(echo, chad_link);
	app::run(main_page, None)?;
	Ok(())
}

enum YardId {
	AssetsTab,
	FactionsTab,
	FactionsList,
	AssetList,
}

impl YardId {
	pub fn as_i32(&self) -> i32 {
		match self {
			YardId::AssetsTab => 700,
			YardId::FactionsTab => 701,
			YardId::FactionsList => 702,
			YardId::AssetList => 703,
		}
	}
}