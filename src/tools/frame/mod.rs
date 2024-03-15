use std::ops::Range;

use crate::tools::inset::Inset;

pub mod layout;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Frame {
	pub left: i16,
	pub right: i16,
	pub top: i16,
	pub bottom: i16,
	pub z: i16,
}

impl Frame {
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
	pub fn split_from_top(mut self, rows: u16) -> (Frame, Frame) {
		let split = self.top + rows as i16;
		let bottom = Frame { top: split, ..self.clone() };
		self.bottom = split;
		(self, bottom)
	}
	pub fn split_from_right(mut self, cols: u16) -> (Frame, Frame) {
		let split = self.right - cols as i16;
		let left_frame = Frame { right: split, ..self.clone() };
		self.left = split;
		(self, left_frame)
	}
	pub fn into_single_row_full_width_at_top(self, top_row: i16) -> Frame {
		Frame { top: top_row, bottom: top_row + 1, ..self }
	}

	pub fn into_single_row_full_width_shift_rows_down(self, rows: u16) -> Frame {
		let next_top = self.top + rows as i16;
		Frame { top: next_top, bottom: next_top + 1, ..self }
	}
	pub fn into_single_row_fixed_width_centered(self, width: u16) -> Frame {
		let width = width as i16;
		let left = self.left + (self.width() - width) / 2;
		let top = self.top + (self.bottom - self.top) / 2 - 1;
		Frame { left, right: left + width, top, bottom: top + 1, z: self.z }
	}
	pub fn into_single_row_fixed_width_at_offset_from_bottom_right(self, width: u16, right_offset: u16, bottom_offset: u16) -> Frame {
		let right = self.right - (right_offset as i16);
		let left = right - (width as i16);
		let bottom = self.bottom - (bottom_offset as i16);
		let top = bottom - 1;
		Frame { left, right, top, bottom, z: self.z }
	}
	pub fn with_z(mut self, z: i16) -> Self {
		self.z = z;
		self
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
	pub fn with_width_from_left(mut self, width_cols: u16) -> Self {
		self.right = self.left + width_cols as i16;
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
	pub fn col_range(&self) -> Range<i16> { self.left..self.right }
	pub fn row_range(&self) -> Range<i16> { self.top..self.bottom }

	pub fn get_row_kind(&self, row: i16) -> RowKind {
		if row < (self.top - 1) {
			RowKind::Above
		} else if row == (self.top - 1) {
			RowKind::TopRail
		} else if row < self.bottom {
			RowKind::Interior
		} else if row == self.bottom {
			RowKind::BottomRail
		} else {
			RowKind::Below
		}
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RowKind {
	Above,
	TopRail,
	Interior,
	BottomRail,
	Below,
}