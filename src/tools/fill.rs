use crate::tools::frame::Frame;

pub fn string_to_fills(string: &str, zrect: Frame, color_index: usize) -> Vec<Fill> {
	let mut fill_zrect = zrect.with_width_from_left(1).with_height_from_top(1);
	let mut vec = Vec::new();
	let chars = string.chars().collect::<Vec<_>>();
	for i in 0..chars.len() {
		let char = chars[i];
		if !char.is_whitespace() && !char.is_control() {
			vec.push(Fill {
				glyph: Glyph::Rune(char, color_index),
				frame: fill_zrect.clone(),
			});
		}
		fill_zrect = fill_zrect.move_right(1);
	}
	vec
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Glyph {
	Rune(char, usize),
	Tile(usize),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Fill {
	pub glyph: Glyph,
	pub frame: Frame,
}

impl Fill {
	pub fn left(&self) -> f32 {
		self.frame.left as f32
	}
	pub fn top(&self) -> f32 {
		self.frame.top as f32
	}
	pub fn width(&self) -> f32 {
		self.frame.width() as f32
	}
	pub fn height(&self) -> f32 {
		self.frame.height() as f32
	}
	pub fn near(&self) -> f32 {
		self.frame.z as f32
	}
}
