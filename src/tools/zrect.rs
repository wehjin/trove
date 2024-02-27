use crate::tools::inset::Inset;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct ZRect {
	pub left: i16,
	pub right: i16,
	pub top: i16,
	pub bottom: i16,
	pub z: i16,
}

impl ZRect {
	pub fn from_cols_rows_z(cols: u16, rows: u16, z: i16) -> Self {
		Self { left: 0, top: 0, right: cols as i16, bottom: rows as i16, z }
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
	pub fn into_single_row_fixed_width_centered(self, width: u16) -> ZRect {
		let width = width as i16;
		let left = self.left + (self.width() - width) / 2;
		let top = self.top + (self.bottom - self.top) / 2 - 1;
		ZRect { left, right: left + width, top, bottom: top + 1, z: self.z }
	}
	pub fn into_single_row_fixed_width_at_offset_from_bottom_right(self, width: u16, right_offset: u16, bottom_offset: u16) -> ZRect {
		let right = self.right - (right_offset as i16);
		let left = right - (width as i16);
		let bottom = self.bottom - (bottom_offset as i16);
		let top = bottom - 1;
		ZRect { left, right, top, bottom, z: self.z }
	}
	pub fn move_down(mut self, rows: u16) -> Self {
		self.top += rows as i16;
		self.bottom += rows as i16;
		self
	}
	pub fn move_closer(self, layers: u16) -> Self {
		Self { z: self.z + layers as i16, ..self }
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
