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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Fill {
	pub glyph: Glyph,
	pub volume: Frame,
}

impl Fill {
	pub fn left(&self) -> f32 {
		self.volume.left as f32
	}
	pub fn top(&self) -> f32 {
		self.volume.top as f32
	}
	pub fn width(&self) -> f32 {
		self.volume.width() as f32
	}
	pub fn height(&self) -> f32 {
		self.volume.height() as f32
	}
	pub fn near(&self) -> f32 {
		self.volume.z as f32
	}
}
