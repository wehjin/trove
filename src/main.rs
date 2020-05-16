extern crate yui;

use std::error::Error;

use yui::{Cling, Confine, Pack, Padding, StringEdit, yard};
use yui::palette::StrokeColor;

fn main() -> Result<(), Box<dyn Error>> {
	yui::Projector::run_blocking(|projector| {
		let strands = vec![
			yard::textfield(2000, "Custodian", StringEdit::empty(), |_| {}),
			yard::textfield(2001, "Account", StringEdit::empty(), |_| {}),
			yard::textfield(2002, "Symbol", StringEdit::empty(), |_| {}),
			yard::textfield(2003, "Shares", StringEdit::empty(), |_| {}),
			yard::textfield(2004, "Corral", StringEdit::empty(), |_| {}),
			yard::textfield(2005, "Price", StringEdit::empty(), |_| {}),
		];
		let strand_height = 3;
		let strand_gap = 1;
		let trellis_height = (strand_height + strand_gap) * strands.len() as i32 - 1;
		let trellis = yard::trellis(strand_height, strand_gap, strands);
		let title = yard::title("Add Asset", StrokeColor::BodyOnBackground, Cling::LeftTop);
		let button = yard::button("Done", |_| {});
		let item = yard::empty()
			.pack_top(3, button.confine(8, 1, Cling::LeftBottom))
			.pack_top(trellis_height, trellis)
			.pack_top(4, title)
			.pad(1);
		projector.set_yard(item);
	});
	println!("Hello, world!");
	Ok(())
}
