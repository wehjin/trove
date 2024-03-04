use std::collections::HashMap;

use rand::random;

use ScrollListMsg::SetFocus;

use crate::tools::captor::{Captor, CaptorId};
use crate::tools::fill::{Fill, string_to_fills};
use crate::tools::frame::{Frame, RowKind};
use crate::tools::inset::Inset;
use crate::tools::solar_dark;
use crate::tools::views::{EdgeHolder, ZMax};

pub struct ScrollListRowDisplay {
	pub col1: String,
	pub col2: String,
	pub col3: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ScrollListMsg {
	SetFocus(CaptorId),
}

pub struct ScrollList {
	id: u64,
	rows: Vec<ScrollListRowDisplay>,
	frame: Frame,
	cursor_position: CursorPosition,
}

impl ScrollList {
	pub fn new(rows: Vec<ScrollListRowDisplay>) -> Self {
		Self {
			id: random(),
			rows,
			frame: Frame::default(),
			cursor_position: CursorPosition::new(0),
		}
	}
	pub fn update(&mut self, msg: ScrollListMsg) {
		match msg {
			SetFocus(CaptorId(_group, index)) => self.set_focus(index),
		}
	}
}

impl EdgeHolder for ScrollList {
	fn set_edge(&mut self, frame: Frame) -> ZMax {
		self.frame = frame;
		self.cursor_position.set_frame(frame);
		// TODO ZMax should include z of rows.
		ZMax(frame.z)
	}
}

impl ScrollList {
	pub fn get_fills_captors(&self, active_captor_id: Option<CaptorId>) -> (Vec<Fill>, Vec<Captor<ScrollListMsg>>) {
		let (mut fills, mut captors) = (Vec::new(), Vec::new());
		for item_index in 0..self.rows.len() {
			let (item_row, kind) = self.cursor_position.get_item_row_and_kind(item_index);
			let item_frame = self.frame
				.inset(Inset::Cols(1))
				.into_single_row_full_width_at_top(item_row)
				.move_closer(1);
			let item_captor_id = CaptorId(self.id, item_index);
			match kind {
				RowKind::TopRail | RowKind::Interior | RowKind::BottomRail => {
					let captor = Captor {
						id: item_captor_id,
						event_map: HashMap::new(),
						frame: item_frame,
						pre_focus_msg: SetFocus(item_captor_id),
					};
					captors.push(captor);
				}
				_ => {}
			}
			if let RowKind::Interior = kind {
				let item_has_focus = active_captor_id == Some(item_captor_id);
				let item_fills = Self::get_item_fills(&self.rows[item_index], item_frame, item_has_focus);
				fills.extend(item_fills);
			}
		}
		(fills, captors)
	}

	fn get_item_fills(row: &ScrollListRowDisplay, item_frame: Frame, with_focus: bool) -> Vec<Fill> {
		const TAB_STOPS: [u16; 3] = [20, 40, 60];
		const NORMAL_PALETTE: usize = 0;
		const FOCUS_PALETTE: usize = 1;
		const NAME_COLOR: usize = 0;
		const KIND_COLOR: usize = 1;
		const SYMBOL_COLOR: usize = 2;
		const BACKGROUND_COLOR: usize = 3;
		const COLORS: [[usize; 4]; 2] = [
			[solar_dark::BASE1, solar_dark::BASE0, solar_dark::BASE01, solar_dark::BASE03],
			[solar_dark::BASE01, solar_dark::BASE00, solar_dark::BASE1, solar_dark::BASE3],
		];
		let palette = match with_focus {
			true => FOCUS_PALETTE,
			false => NORMAL_PALETTE
		};
		let mut fills = Vec::new();
		let background_frame = item_frame;
		{
			let background_fill = Fill::color_tile(background_frame, COLORS[palette][BACKGROUND_COLOR]);
			fills.push(background_fill);
		}
		let text_frame = background_frame.move_closer(1);
		{
			let name_frame = text_frame.with_width_from_left(TAB_STOPS[0]);
			let name_fills = string_to_fills(row.col1.as_str(), name_frame, COLORS[palette][NAME_COLOR]);
			fills.extend(name_fills);
			let kind_frame = text_frame.inset(Inset::Left(TAB_STOPS[0] as u8)).with_width_from_left(TAB_STOPS[1] - TAB_STOPS[0]);
			let kind_fills = string_to_fills(row.col2.as_str(), kind_frame, COLORS[palette][KIND_COLOR]);
			fills.extend(kind_fills);
			let symbol_frame = text_frame.inset(Inset::Left(TAB_STOPS[1] as u8)).with_width_from_left(TAB_STOPS[2] - TAB_STOPS[1]);
			let symbol_fills = {
				let symbol = if row.col3.is_empty() { "<???>" } else { row.col3.as_str() };
				string_to_fills(symbol, symbol_frame, COLORS[palette][SYMBOL_COLOR])
			};
			fills.extend(symbol_fills);
		}
		fills
	}
}

impl ScrollList {
	pub fn add_row(&mut self, new_row: ScrollListRowDisplay) {
		self.rows.push(new_row);
		self.cursor_position.set_len_show_last(self.rows.len());
	}
	pub fn set_focus(&mut self, index: usize) {
		self.cursor_position.set_cursor_index(index);
	}
}

#[derive(Debug, Copy, Clone)]
pub struct CursorPosition {
	list_len: usize,
	top_index: usize,
	cursor_index: usize,
	frame: Frame,
}

impl CursorPosition {
	pub fn new(list_len: usize) -> Self {
		Self {
			list_len,
			top_index: 0,
			cursor_index: 0,
			frame: Frame::default(),
		}
	}
	pub fn set_frame(&mut self, frame: Frame) {
		let cursor_offset = self.cursor_index - self.top_index;
		if cursor_offset >= frame.height().max(0) as usize {
			self.top_index = ((frame.bottom as isize - 1) - cursor_offset as isize).max(0) as usize;
		}
		self.frame = frame;
	}
	pub fn set_cursor_index(&mut self, index: usize) {
		let max_cursor = ((self.list_len as isize) - 1).max(0) as usize;
		let index = index.min(max_cursor);
		let bottom_index = self.top_index + self.frame.height().max(0) as usize;
		if index < self.top_index {
			self.cursor_index = index;
			self.top_index = index;
		} else if index < bottom_index {
			self.cursor_index = index;
		} else {
			self.cursor_index = index;
			self.top_index = (index as isize + 1 - self.frame.height().max(0) as isize).max(0) as usize;
		}
	}
	pub fn set_len_show_last(&mut self, new_len: usize) {
		self.list_len = new_len;
		self.set_cursor_index(new_len);
	}
	fn get_cursor_row(&self) -> i16 {
		self.frame.top + (self.cursor_index - self.top_index) as i16
	}
	pub fn get_item_row_and_kind(&self, item_index: usize) -> (i16, RowKind) {
		let cursor_row = self.get_cursor_row();
		let item_row = {
			let item_offset = (item_index as isize) - (self.cursor_index as isize);
			((cursor_row as isize) + item_offset) as i16
		};
		(item_row, self.frame.get_row_kind(item_row))
	}
}
