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
	Release,
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
			FabMsg::Press => if !self.pressed {
				self.pressed = true;
			},
			FabMsg::Release => if self.pressed {
				self.pressed = false;
			}
		}
	}
	fn get_fills(&self, edge_frame: Frame) -> Vec<Fill> {
		let back_fill = vec![Fill::color_tile(edge_frame, solar_dark::BASE3)];
		let label_fills = string_to_fills(self.init.0.as_str(), edge_frame.move_closer(1), solar_dark::BASE00);
		vec![back_fill, label_fills].into_iter().flatten().collect()
	}
}

impl Fab {
	pub fn min_width_height(&self) -> (u16, u16) {
		let min_cols = self.init.0.as_str().chars().count().max(3) as u16;
		(min_cols, 1)
	}
	fn captor(_frame: Frame) -> Captor<FabMsg> {
		let mut event_map = HashMap::new();
		event_map.insert(UserEvent::PressStart, FabMsg::Press);
		event_map.insert(UserEvent::PressEnd, FabMsg::Release);
		Captor { event_map }
	}
}
