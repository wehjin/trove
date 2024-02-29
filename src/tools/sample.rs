use crate::tools::{solar_dark, View, ViewChanging};
use crate::tools::fill::{Fill, string_to_fills};
use crate::tools::frame::Frame;
use crate::tools::inset::Inset;
use crate::tools::ViewStarting;

pub struct SampleAppInit;

impl ViewStarting for SampleAppInit {
	type Model = SampleApp;
	fn init(self) -> Self::Model {
		SampleApp
	}
}

pub struct SampleApp;

impl ViewChanging for SampleApp {
	type Msg = ();
	fn update(&mut self, _msg: Self::Msg) {}
}

impl View for SampleApp {
	fn get_fills(&self, edge_frame: Frame) -> Vec<Fill> {
		let edge_frame = edge_frame.inset(Inset::DoubleCols(1)).move_closer(1);
		const TITLE_TEXT: &str = "Assets";
		const EMPTY_TEXT: &str = "Empty in assets";
		const BUTTON_LABEL: &str = "{+}";
		let (title_frame, body_frame) = edge_frame.split_from_top(1);
		let empty_text_frame = body_frame.into_single_row_fixed_width_centered(EMPTY_TEXT.chars().count() as u16).move_closer(1);
		let button_frame = body_frame.into_single_row_fixed_width_at_offset_from_bottom_right(BUTTON_LABEL.chars().count() as u16, 2, 1).move_closer(1);
		let back_fills = vec![
			Fill::color_tile(title_frame, solar_dark::BASE02),
			Fill::color_tile(body_frame, solar_dark::BASE03),
			Fill::color_tile(button_frame, solar_dark::BASE3),
		];
		let title_fills = string_to_fills(TITLE_TEXT, title_frame.move_closer(1).inset(Inset::Cols(2)), solar_dark::BASE1);
		let empty_text_fills = string_to_fills(EMPTY_TEXT, empty_text_frame, solar_dark::BASE0);
		let button_text_fills = string_to_fills(BUTTON_LABEL, button_frame.move_closer(1), solar_dark::BASE00);
		vec![back_fills, title_fills, empty_text_fills, button_text_fills].into_iter().flatten().collect()
	}
}
