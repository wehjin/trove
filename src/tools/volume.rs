use crate::tools::inset::Inset;

#[derive(Copy, Clone, Default)]
pub struct ZRect {
	pub left: i16,
	pub right: i16,
	pub top: i16,
	pub bottom: i16,
	pub z: i16,
}

impl ZRect {
	pub fn from_cols_rows_near(cols: u16, rows: u16, near: i16) -> Self {
		Self { left: 0, top: 0, right: cols as i16, bottom: rows as i16, z: near }
	}
	pub fn inset(mut self, inset: Inset) -> Self {
		let (t, r, b, l) = inset.to_top_right_bottom_left();
		self.top += t as i16;
		self.right -= r as i16;
		self.bottom -= b as i16;
		self.left += l as i16;
		self
	}

	pub fn split_from_top(mut self, rows: u16) -> (ZRect, ZRect) {
		let split = self.top + rows as i16;
		let bottom = ZRect { top: split, ..self.clone() };
		self.bottom = split;
		(self, bottom)
	}
	pub fn move_closer(mut self, layers: u16) -> Self {
		self.z += layers as i16;
		self
	}
	pub fn move_right(mut self, cols: u16) -> Self {
		self.right += cols as i16;
		self.left += cols as i16;
		self
	}
	pub fn with_width_from_left(mut self, width: u16) -> Self {
		self.right = self.left + width as i16;
		self
	}
	pub fn with_height_from_top(mut self, height: u16) -> Self {
		self.bottom = self.top + height as i16;
		self
	}
	pub fn width(&self) -> i16 {
		self.right - self.left
	}
	pub fn height(&self) -> i16 {
		self.bottom - self.top
	}
}
