use std::collections::HashMap;

use crate::tools::{Captor, solar_dark, UserEvent};
use crate::tools::fill::{Fill, string_to_fills};
use crate::tools::frame::Frame;
use crate::tools::views::{View, ViewStarting};

pub struct FabLabel(pub String);

impl Default for FabLabel {
	fn default() -> Self { Self("todo".into()) }
}

impl ViewStarting for FabLabel {
	type Model = Fab;

	fn into_view(self) -> Self::Model {
		let model = Fab { init: self, ..Fab::default() };
		model
	}
}

#[derive(Copy, Clone, Debug)]
pub enum FabMsg {
	Press,
}

#[derive(Default)]
pub struct Fab {
	init: FabLabel,
	pressed: bool,
}

impl View for Fab {
	type Msg = FabMsg;

	fn update(&mut self, msg: Self::Msg) {
		match msg {
			FabMsg::Press => {
				// Since terminal does not have concept of key press and release, for now,
				// just toggle the state with each press.
				// TODO add a timer to animate the press and then signal the parent that button was pressed.
				self.pressed = !self.pressed;
			}
		}
	}
	fn get_fills_captors(&self, edge_frame: Frame) -> (Vec<Fill>, Vec<Captor<FabMsg>>) {
		let (back_color, label_color) = match self.pressed {
			true => (solar_dark::BASE1, solar_dark::BASE02),
			false => (solar_dark::BASE3, solar_dark::BASE00),
		};
		let back_fill = vec![Fill::color_tile(edge_frame, back_color)];
		let label_fills = string_to_fills(self.init.0.as_str(), edge_frame.move_closer(1), label_color);
		let fills = vec![back_fill, label_fills].into_iter().flatten().collect::<Vec<_>>();
		let captors = {
			let mut event_map = HashMap::new();
			event_map.insert(UserEvent::Select, FabMsg::Press);
			let frame = edge_frame;
			vec![Captor { event_map, frame }]
		};
		(fills, captors)
	}
}

impl Fab {
	pub fn min_width_height(&self) -> (u16, u16) {
		let min_cols = self.init.0.as_str().chars().count().max(3) as u16;
		(min_cols, 1)
	}
}
