use crate::components::fill::Fill;
use crate::tools::frame::Frame;

pub fn string_to_fills(string: &str, zrect: Frame, color_index: usize) -> Vec<Fill> {
	let mut fill_zrect = zrect.with_width_from_left(1).with_height_from_top(1);
	let mut vec = Vec::new();
	for i in 0..string.chars().count() {
		if !&string[i..i + 1].trim().is_empty() {
			let fill = Fill { glyph: Glyph::Text(color_index), volume: fill_zrect.clone() };
			vec.push(fill);
		}
		fill_zrect = fill_zrect.move_right(1);
	}
	vec
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Glyph {
	Solid(usize),
	Text(usize),
}
