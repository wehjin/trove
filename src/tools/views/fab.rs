use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use crate::tools::{Cmd, solar_dark, UserEvent};
use crate::tools::captor::Captor;
use crate::tools::fill::{Fill, string_to_fills};
use crate::tools::frame::Frame;

#[derive(Debug, Copy, Clone)]
pub enum FabMsg {
	Press,
	Release,
}

#[derive(Debug, Default)]
pub struct Fab {
	pub label: String,
	pub pressed: bool,
}

impl Fab {
	pub fn update(&mut self, msg: FabMsg) -> Cmd<FabMsg> {
		let cmd = match msg {
			FabMsg::Press if !self.pressed => {
				self.pressed = true;
				Cmd::Unit(Box::new(|| {
					thread::sleep(Duration::from_millis(100));
					FabMsg::Release
				}))
			}
			FabMsg::Release if self.pressed => {
				self.pressed = false;
				// TODO Notify caller.
				Cmd::None
			}
			_ => Cmd::None
		};
		cmd
	}
	pub fn get_fills_captors(&self, edge_frame: Frame) -> (Vec<Fill>, Vec<Captor<FabMsg>>) {
		let (back_color, label_color) = match self.pressed {
			false => (solar_dark::BASE3, solar_dark::BASE00),
			true => (solar_dark::BASE1, solar_dark::BASE02),
		};
		let back_fill = vec![Fill::color_tile(edge_frame, back_color)];
		let label_fills = string_to_fills(self.label.as_str(), edge_frame.move_closer(1), label_color);
		let fills = vec![back_fill, label_fills].into_iter().flatten().collect::<Vec<_>>();
		let captors = {
			let mut event_map = HashMap::new();
			event_map.insert(UserEvent::Select, FabMsg::Press);
			let frame = edge_frame;
			vec![Captor { event_map, frame }]
		};
		(fills, captors)
	}


	pub fn min_width_height(&self) -> (u16, u16) {
		let min_cols = self.label.as_str().chars().count().max(3) as u16;
		(min_cols, 1)
	}
}
