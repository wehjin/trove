use crate::components::fill::Fill;
use crate::tools::Volume;

pub fn string_to_fills(string: &str, string_volume: Volume, color_index: usize) -> Vec<Fill> {
	let mut fill_volume = string_volume.with_width_from_left(1).with_height_from_top(1);
	let mut vec = Vec::new();
	for i in 0..string.chars().count() {
		if !&string[i..i + 1].trim().is_empty() {
			let fill = Fill { glyph: Glyph::Text(color_index), volume: fill_volume.clone() };
			vec.push(fill);
		}
		fill_volume = fill_volume.move_right(1);
	}
	vec
}

#[derive(Clone)]
pub enum Glyph {
	Solid(usize),
	Text(usize),
}
