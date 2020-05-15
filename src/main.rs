extern crate yui;

use std::error::Error;

use yui::{Cling, Confine, Pack, Padding, yard};
use yui::palette::StrokeColor;

fn main() -> Result<(), Box<dyn Error>> {
	yui::Projector::run_blocking(|projector| {
		let left_label = yard::label("Amazon", StrokeColor::BodyOnBackground, Cling::Left);
		let left_color = yard::label("sovereign:main", StrokeColor::CommentOnBackground, Cling::Left);
		let left_side = left_label.pack_bottom(1, left_color);
		let item = left_side.confine_height(2, Cling::Top).pad(1);
		projector.set_yard(item);
	});
	println!("Hello, world!");
	Ok(())
}
