use crate::tools::{Cmd, solar_dark};
use crate::tools::captor::Captor;
use crate::tools::fill::{Fill, string_to_fills};
use crate::tools::frame::Frame;
use crate::tools::inset::Inset;
use crate::tools::sample::SampleAppMsg::ForFab;
use crate::tools::views::fab::{Fab, FabMsg};

#[derive(Copy, Clone, Debug)]
pub enum SampleAppMsg {
	ForFab(FabMsg)
}

pub struct SampleApp {
	pub fab: Fab,
}

impl SampleApp {
	pub fn update(&mut self, msg: SampleAppMsg) -> Cmd<SampleAppMsg> {
		match msg {
			ForFab(msg) => self.fab.update(msg).map(ForFab),
		}
	}
	pub fn get_fills_captors(&self, edge_frame: Frame) -> (Vec<Fill>, Vec<Captor<SampleAppMsg>>) {
		const EMPTY_TEXT: &str = "Empty in assets";
		let edge_frame = edge_frame.inset(Inset::DoubleCols(1)).move_closer(1);
		let (title_frame, body_frame) = edge_frame.split_from_top(1);
		let empty_text_frame = body_frame.into_single_row_fixed_width_centered(EMPTY_TEXT.chars().count() as u16).move_closer(1);
		let fab_frame = body_frame.into_single_row_fixed_width_at_offset_from_bottom_right(self.fab.min_width_height().0, 2, 1).move_closer(1);
		let title_body_fills = vec![
			Fill::color_tile(title_frame, solar_dark::BASE02),
			Fill::color_tile(body_frame, solar_dark::BASE03),
		];
		let title_fills = string_to_fills("Assets", title_frame.move_closer(1).inset(Inset::Cols(2)), solar_dark::BASE1);
		let empty_text_fills = string_to_fills(EMPTY_TEXT, empty_text_frame, solar_dark::BASE01);

		let (fab_fills, fab_captors) = self.fab.get_fills_captors(fab_frame);
		let fills = vec![title_body_fills, title_fills, empty_text_fills, fab_fills].into_iter().flatten().collect::<Vec<_>>();
		let captors = fab_captors.into_iter().map(|it| it.map_msg(SampleAppMsg::ForFab)).collect::<Vec<_>>();
		(fills, captors)
	}
}
