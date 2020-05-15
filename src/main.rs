extern crate yui;

use std::error::Error;

use yui::{Cling, Confine, Pack, Padding, yard};
use yui::palette::StrokeColor;

fn main() -> Result<(), Box<dyn Error>> {
	yui::Projector::run_blocking(|projector| {
		let custodian = yard::textfield("Custodian");
		let account = yard::textfield("Account");
		let symbol = yard::textfield("Symbol");
		let shares = yard::textfield("Shares");
		let group = yard::textfield("Corral");
		let price = yard::textfield("Price");

		let rung_count = 6;
		let ladder_height = 4 * rung_count - 1;
		let ladder = custodian
			.pack_bottom(4, account.confine_height(3, Cling::LeftBottom))
			.pack_bottom(4, symbol.confine_height(3, Cling::LeftBottom))
			.pack_bottom(4, shares.confine_height(3, Cling::LeftBottom))
			.pack_bottom(4, group.confine_height(3, Cling::LeftBottom))
			.pack_bottom(4, price.confine_height(3, Cling::LeftBottom));
		let title = yard::label("Add Account", StrokeColor::BodyOnBackground, Cling::LeftTop);
		let button = yard::button("Done", |_| {});
		let item = yard::empty()
			.pack_top(3, button.confine(8, 1, Cling::LeftBottom))
			.pack_top(ladder_height, ladder)
			.pack_top(3, yard::label("===========", StrokeColor::BodyOnBackground, Cling::LeftTop))
			.pack_top(1, title)
			.pad(1);
		projector.set_yard(item);
	});
	println!("Hello, world!");
	Ok(())
}
