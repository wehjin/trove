use rand::random;

use crate::tools::{Cmd, solar_dark};
use crate::tools::captor::Captor;
use crate::tools::fill::{Fill, string_to_fills};
use crate::tools::frame::Frame;
use crate::tools::inset::Inset;
use crate::tools::sample::SampleAppMsg::ForFab;
use crate::tools::views::fab::{Fab, FabMsg, JustClicked};

#[derive(Copy, Clone, Debug)]
pub enum SampleAppMsg {
	ForFab(FabMsg)
}

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
	id: u64,
	name: String,
	kind: AssetKind,
	symbol: String,
}

impl Asset {
	pub fn new() -> Self {
		Self {
			id: random(),
			name: "New Asset".to_string(),
			kind: AssetKind::Commodity,
			symbol: "".to_string(),
		}
	}
}

pub struct SampleApp {
	pub fab: Fab,
	pub asset_list: Vec<Asset>,
}

impl SampleApp {
	pub fn new() -> Self {
		Self {
			fab: Fab { label: " [+] ".to_string(), ..Fab::default() },
			asset_list: vec![],
		}
	}
	pub fn update(&mut self, msg: SampleAppMsg) -> Cmd<SampleAppMsg> {
		match msg {
			ForFab(msg) => match self.fab.update(msg) {
				JustClicked::Yes => {
					self.asset_list.push(Asset::new());
					Cmd::None
				}
				JustClicked::No(cmd) => cmd.map(ForFab)
			},
		}
	}
	pub fn get_fills_captors(&self, edge_frame: Frame) -> (Vec<Fill>, Vec<Captor<SampleAppMsg>>) {
		const EMPTY_TEXT: &str = "Empty in assets";
		let edge_frame = edge_frame.inset(Inset::DoubleCols(1)).move_closer(1);
		let (title_frame, body_frame) = edge_frame.split_from_top(1);
		let fab_frame = body_frame.into_single_row_fixed_width_at_offset_from_bottom_right(self.fab.min_width_height().0, 2, 1).move_closer(1);
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
		let list_fills = if self.asset_list.len() > 0 {
			let mut fills = Vec::new();
			for i in 0..self.asset_list.len() {
				let asset = &self.asset_list[i];
				let tab_stop = [20, 40, 60];
				let item_frame = body_frame
					.inset(Inset::Cols(1))
					.into_single_row_full_width_shift_rows_down(i as u16)
					.move_closer(1);
				let name_frame = item_frame.with_width_from_left(tab_stop[0]);
				fills.append(&mut string_to_fills(asset.name.as_str(), name_frame, solar_dark::BASE1));
				let kind_frame = item_frame.inset(Inset::Left(tab_stop[0] as u8)).with_width_from_left(tab_stop[1] - tab_stop[0]);
				fills.append(&mut string_to_fills(asset.kind.as_str(), kind_frame, solar_dark::BASE0));
				let symbol_frame = item_frame.inset(Inset::Left(tab_stop[1] as u8)).with_width_from_left(tab_stop[2] - tab_stop[1]);
				let symbol = if asset.symbol.is_empty() { "<???>" } else { asset.symbol.as_str() };
				fills.append(&mut string_to_fills(symbol, symbol_frame, solar_dark::BASE01))
			}
			fills
		} else {
			vec![]
		};
		let (fab_fills, fab_captors) = self.fab.get_fills_captors(fab_frame);
		let fills = vec![
			title_body_fills,
			title_fills,
			empty_text_fills,
			fab_fills,
			list_fills,
		].into_iter().flatten().collect::<Vec<_>>();
		let captors = fab_captors.into_iter().map(|it| it.map_msg(SampleAppMsg::ForFab)).collect::<Vec<_>>();
		(fills, captors)
	}
}
