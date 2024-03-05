use rand::random;

use crate::data::Asset;
use crate::tools::{Cmd, solar_dark};
use crate::tools::captor::{Captor, CaptorId};
use crate::tools::fill::{Fill, string_to_fills};
use crate::tools::frame::Frame;
use crate::tools::frame::layout::Layout;
use crate::tools::inset::Inset;
use crate::tools::sample::SampleAppMsg::{ForFab, ForScrollList};
use crate::tools::views::EdgeHolder;
use crate::tools::views::fab::{Fab, FabMsg, JustClicked};
use crate::tools::views::scroll_list::{JustSelected, ScrollList, ScrollListMsg};

#[derive(Copy, Clone, Debug)]
pub enum SampleAppMsg {
	ForFab(FabMsg),
	ForScrollList(ScrollListMsg),
}

pub struct SampleApp {
	pub id: u64,
	pub asset_list: Vec<Asset>,
	pub scroll_list: ScrollList,
	pub fab: Fab,
	pub title_frame: Frame,
	pub body_frame: Frame,
}


impl SampleApp {
	pub fn new() -> Self {
		Self {
			id: random(),
			asset_list: vec![],
			scroll_list: ScrollList::new(vec![]),
			fab: Fab { label: " [+] ".to_string(), ..Fab::default() },
			title_frame: Frame::default(),
			body_frame: Frame::default(),
		}
	}
	pub fn update_with_effects(&mut self, msg: SampleAppMsg) -> Cmd<SampleAppMsg> {
		match msg {
			ForFab(msg) => match self.fab.update(msg) {
				JustClicked::Yes => {
					let asset = Asset::new(self.asset_list.len() + 1);
					let asset_row = asset.to_row_display();
					self.asset_list.push(asset);
					self.scroll_list.add_row(asset_row);
					Cmd::None
				}
				JustClicked::No(cmd) => cmd.map(ForFab),
			},
			ForScrollList(msg) => {
				let row_selected = self.scroll_list.update_with_event(msg);
				match row_selected {
					JustSelected::None => {}
					JustSelected::Row(_index) => {}
				}
				Cmd::None
			}
		}
	}
	pub fn set_edge_frame(&mut self, edge_frame: Frame) -> i16 {
		Layout::new(edge_frame)
			.inset(Inset::DoubleCols(1))
			.move_closer(1)
			.split_top(1).take(&mut self.title_frame)
			.take(&mut self.body_frame)
		;
		let z_max = self.scroll_list.set_edge(self.body_frame);
		let z_max = (z_max + 5).z();
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
			let (fills, captors) = self.scroll_list.get_fills_captors(active_captor_id);
			let captors = captors.into_iter().map(|captor| captor.map_msg(ForScrollList)).collect::<Vec<_>>();
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
}
