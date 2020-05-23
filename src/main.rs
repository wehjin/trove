extern crate dirs;
extern crate echo_lib;
extern crate rand;
extern crate stringedit;
extern crate yui;

use std::error::Error;

use yui::app;

use crate::list_assets::ListAssets;

mod asset_edit;
mod data;

#[derive(Debug, Clone)]
pub struct QuadText {
	title: String,
	subtitle: String,
	value: String,
	subvalue: String,
}

impl QuadText {
	pub fn title(&self) -> &String { &self.title }
	pub fn subtitle(&self) -> &String { &self.subtitle }
	pub fn value(&self) -> &String { &self.value }
	pub fn subvalue(&self) -> &String { &self.subvalue }
}

mod list_assets;


fn main() -> Result<(), Box<dyn Error>> {
	let echo = data::echo()?;
	let list_assets = ListAssets::new(&echo);
	app::run(list_assets, None)?;
	Ok(())
}
