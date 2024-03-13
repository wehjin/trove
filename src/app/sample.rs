use rand::random;

use crate::app::details::{Details, DetailsMsg};
use crate::app::sample::SampleAppMsg::{ForDetails, ForFab, ForScrollList};
use crate::data::Asset;
use crate::tools::{Cmd, solar_dark};
use crate::tools::captor::{Captor, CaptorId};
use crate::tools::fill::{Fill, string_to_fills};
use crate::tools::frame::Frame;
use crate::tools::frame::layout::Layout;
use crate::tools::inset::Inset;
use crate::tools::views::{Shaping, Updating, Viewing, ZMax};
use crate::tools::views::fab::{Fab, FabMsg, JustClicked};
use crate::tools::views::scroll_list::{JustSelected, ScrollList, ScrollListMsg};

#[derive(Debug, Clone)]
pub enum SampleAppMsg {
	ForFab(FabMsg),
	ForScrollList(ScrollListMsg),
	ForDetails(DetailsMsg),
}

pub struct SampleApp {
	pub id: u64,
	pub asset_list: Vec<Asset>,
	pub selected_asset: Option<usize>,
	pub scroll_list: ScrollList,
	pub fab: Fab,
	pub details: Details,
	pub title_frame: Frame,
	pub list_frame: Frame,
}

impl SampleApp {
	pub fn new() -> Self {
		Self {
			id: random(),
			asset_list: vec![],
			selected_asset: None,
			scroll_list: ScrollList::new(vec![]),
			fab: Fab { label: " [+] ".to_string(), ..Fab::default() },
			details: Details::default(),
			title_frame: Frame::default(),
			list_frame: Frame::default(),
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
					JustSelected::Row(index) => {
						self.selected_asset = Some(index);
						self.details.update(DetailsMsg::SetAsset(self.asset_list[index].clone()))
					}
				}
				Cmd::None
			}
			ForDetails(msg) => {
				self.details.update(msg);
				Cmd::None
			}
		}
	}
	pub fn get_fills_captors(&self, active_captor_id: Option<CaptorId>) -> (Vec<Fill>, Vec<Captor<SampleAppMsg>>) {
		const EMPTY_TEXT: &str = "Empty in assets";
		let title_frame = self.title_frame;
		let body_frame = self.list_frame;
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
		let (detail_fills, detail_captors) = self.details.get_fills_captors(active_captor_id);
		let detail_captors = detail_captors
			.into_iter()
			.map(|it| it.map_msg(ForDetails))
			.collect::<Vec<_>>();
		let fills = vec![
			title_body_fills,
			title_fills,
			empty_text_fills,
			list_fills,
			fab_fills,
			detail_fills,
		].into_iter().flatten().collect::<Vec<_>>();
		let captors = vec![
			list_captors,
			fab_captors,
			detail_captors,
		].into_iter().flatten().collect();
		(fills, captors)
	}
}

impl Shaping for SampleApp {
	fn shape(&mut self, frame: Frame) -> ZMax {
		let side_cols = match self.selected_asset {
			None => 0,
			Some(_index) => frame.width() as u16 / 2,
		};
		let z_max = Layout::new(frame)
			.inset(Inset::DoubleCols(1))
			.move_closer(1)
			.split_top(1).take(&mut self.title_frame)
			.split_right(side_cols).move_closer(1).shape(&mut self.details).seal()
			.shape(&mut self.scroll_list)
			.take(&mut self.list_frame)
			.into_z_max();
		let fab_frame = self.list_frame.into_single_row_fixed_width_at_offset_from_bottom_right(self.fab.min_width_height().0, 2, 1).move_closer(1);
		z_max.max(self.fab.shape(fab_frame))
	}
}
