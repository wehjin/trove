use std::collections::HashMap;

use rand::random;

use SampleAppMsg::SetFocus;

use crate::tools::{Cmd, solar_dark};
use crate::tools::captor::{Captor, CaptorId};
use crate::tools::fill::{Fill, string_to_fills};
use crate::tools::frame::{Frame, RowKind};
use crate::tools::inset::Inset;
use crate::tools::sample::SampleAppMsg::ForFab;
use crate::tools::views::fab::{Fab, FabMsg, JustClicked};

#[derive(Debug)]
pub enum AssetKind {
	Commodity,
	Stock,
	Etf,
}

impl AssetKind {
	pub fn as_str(&self) -> &'static str {
		match self {
			AssetKind::Commodity => "Commodity",
			AssetKind::Stock => "Stock",
			AssetKind::Etf => "Etf"
		}
	}
}

pub struct Asset {
	name: String,
	kind: AssetKind,
	symbol: String,
}

impl Asset {
	pub fn new(num: usize) -> Self {
		Self {
			name: "New Asset".to_string(),
			kind: AssetKind::Commodity,
			symbol: format!("CM-{}-{}", num, random::<u16>()),
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub enum SampleAppMsg {
	ForFab(FabMsg),
	SetFocus(CaptorId),
}

#[derive(Debug, Copy, Clone)]
pub enum FocusOffset {
	DownFromTop(usize),
	UpFromBottom(usize),
}

impl FocusOffset {
	pub fn to_row_in_frame(&self, frame: Frame) -> i16 {
		match self {
			FocusOffset::DownFromTop(rows_down) => {
				let row = frame.top + (*rows_down as i16);
				row.min(frame.bottom - 1)
			}
			FocusOffset::UpFromBottom(rows_up) => {
				let row = (frame.bottom - 1) - (*rows_up as i16);
				row.max(frame.top)
			}
		}
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

pub struct SampleApp {
	pub id: u64,
	pub fab: Fab,
	pub asset_list: Vec<Asset>,
	pub cursor_position: CursorPosition,
	pub title_frame: Frame,
	pub body_frame: Frame,
}

impl SampleApp {
	pub fn new() -> Self {
		Self {
			id: random(),
			fab: Fab { label: " [+] ".to_string(), ..Fab::default() },
			asset_list: vec![],
			cursor_position: CursorPosition::new(0),
			title_frame: Frame::default(),
			body_frame: Frame::default(),
		}
	}
	pub fn update(&mut self, msg: SampleAppMsg) -> Cmd<SampleAppMsg> {
		match msg {
			ForFab(msg) => match self.fab.update(msg) {
				JustClicked::Yes => {
					self.asset_list.push(Asset::new(self.asset_list.len() + 1));
					self.cursor_position.set_len_show_last(self.asset_list.len());
					Cmd::None
				}
				JustClicked::No(cmd) => {
					cmd.map(ForFab)
				}
			},
			SetFocus(CaptorId(_group, index)) => {
				self.cursor_position.set_cursor_index(index);
				Cmd::None
			}
		}
	}
	pub fn set_edge_frame(&mut self, edge_frame: Frame) -> i16 {
		let edge_frame = edge_frame.inset(Inset::DoubleCols(1)).move_closer(1);
		(self.title_frame, self.body_frame) = edge_frame.split_from_top(1);
		self.cursor_position.set_frame(self.body_frame);
		let z_max = edge_frame.z + 5;
		let fab_frame = self.body_frame.into_single_row_fixed_width_at_offset_from_bottom_right(self.fab.min_width_height().0, 2, 1).move_closer(1);
		let fab_z_max = self.fab.set_edge_frame(fab_frame);
		z_max.max(fab_z_max)
	}
	pub fn get_fills_captors(&self, active_captor_id: Option<CaptorId>) -> (Vec<Fill>, Vec<Captor<SampleAppMsg>>) {
		const EMPTY_TEXT: &str = "Empty in assets";
		let title_frame = self.title_frame;
		let body_frame = self.body_frame;
		let title_body_fills = vec![
			Fill::color_tile(title_frame, solar_dark::BASE02),
			Fill::color_tile(body_frame, solar_dark::BASE03),
		];
		let title_fills = string_to_fills("Assets", title_frame.move_closer(1).inset(Inset::Cols(2)), solar_dark::BASE1);
		let empty_text_fills = if self.asset_list.is_empty() {
			let empty_text_frame = body_frame.into_single_row_fixed_width_centered(EMPTY_TEXT.chars().count() as u16).move_closer(1);
			string_to_fills(EMPTY_TEXT, empty_text_frame, solar_dark::BASE01)
		} else {
			vec![]
		};
		let (list_fills, list_captors) = {
			let (mut fills, mut captors) = (Vec::new(), Vec::new());
			for item_index in 0..self.asset_list.len() {
				let (item_row, kind) = self.cursor_position.get_item_row_and_kind(item_index);
				let item_frame = body_frame
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
					let item_fills = Self::get_item_fills(&self.asset_list[item_index], item_frame, item_has_focus);
					fills.extend(item_fills);
				}
			}
			(fills, captors)
		};
		let (fab_fills, fab_captors) = self.fab.get_fills_captors(active_captor_id);
		let fab_captors = fab_captors
			.into_iter()
			.map(|it| it.map_msg(ForFab))
			.collect::<Vec<_>>();
		let fills = vec![
			title_body_fills,
			title_fills,
			empty_text_fills,
			list_fills,
			fab_fills,
		].into_iter().flatten().collect::<Vec<_>>();
		let captors = vec![
			list_captors,
			fab_captors,
		].into_iter().flatten().collect();
		(fills, captors)
	}

	fn get_item_fills(asset: &Asset, item_frame: Frame, with_focus: bool) -> Vec<Fill> {
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
			let name_fills = string_to_fills(asset.name.as_str(), name_frame, COLORS[palette][NAME_COLOR]);
			fills.extend(name_fills);
			let kind_frame = text_frame.inset(Inset::Left(TAB_STOPS[0] as u8)).with_width_from_left(TAB_STOPS[1] - TAB_STOPS[0]);
			let kind_fills = string_to_fills(asset.kind.as_str(), kind_frame, COLORS[palette][KIND_COLOR]);
			fills.extend(kind_fills);
			let symbol_frame = text_frame.inset(Inset::Left(TAB_STOPS[1] as u8)).with_width_from_left(TAB_STOPS[2] - TAB_STOPS[1]);
			let symbol_fills = {
				let symbol = if asset.symbol.is_empty() { "<???>" } else { asset.symbol.as_str() };
				string_to_fills(symbol, symbol_frame, COLORS[palette][SYMBOL_COLOR])
			};
			fills.extend(symbol_fills);
		}
		fills
	}
}
