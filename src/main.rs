extern crate stringedit;
extern crate yui;

use std::error::Error;

use yui::app;

use asset_edit::AssetEdit;

mod asset_edit;

fn main() -> Result<(), Box<dyn Error>> {
	app::run::<AssetEdit>()
}



