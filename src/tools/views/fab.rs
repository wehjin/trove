use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use rand::random;

use crate::tools::{Cmd, solar_dark, UserEvent};
use crate::tools::captor::{Captor, CaptorId};
use crate::tools::fill::{Fill, string_to_fills};
use crate::tools::frame::Frame;

#[derive(Debug, Copy, Clone)]
pub enum FabMsg {
	Press,
	Release,
	Ignore,
}

pub fn timer_cmd<T: Send + Sync + 'static>(millis: u64, msg: T) -> Cmd<T> {
	Cmd::Unit(Box::new(move || {
		thread::sleep(Duration::from_millis(millis));
		msg
	}))
}

pub enum JustClicked {
	Yes,
	No(Cmd<FabMsg>),
}

#[derive(Debug)]
pub struct Fab {
	pub id: u64,
	pub label: String,
	pub pressed: bool,
	pub edge_frame: Frame,
}

impl Default for Fab {
	fn default() -> Self {
		Fab {
			id: random(),
			label: "".to_string(),
			pressed: false,
			edge_frame: Frame::default(),
		}
	}
}

impl Fab {
	pub fn update(&mut self, msg: FabMsg) -> JustClicked {
		match msg {
			FabMsg::Press if !self.pressed => {
				self.pressed = true;
				JustClicked::No(timer_cmd(100, FabMsg::Release))
			}
			FabMsg::Release if self.pressed => {
				self.pressed = false;
				JustClicked::Yes
			}
			_ => JustClicked::No(Cmd::None)
		}
	}
	pub fn set_edge_frame(&mut self, edge_frame: Frame) -> i16 {
		self.edge_frame = edge_frame;
		edge_frame.z + 5
	}
	pub fn get_fills_captors(&self, active_captor_id: Option<CaptorId>) -> (Vec<Fill>, Vec<Captor<FabMsg>>) {
		let captor_id = CaptorId(self.id, 0);
		let (back_color, label_color) = match self.pressed {
			true => (solar_dark::BASE3, solar_dark::BASE00),
			false => match active_captor_id == Some(captor_id) {
				true => (solar_dark::BASE2, solar_dark::BASE01),
				false => (solar_dark::BASE02, solar_dark::BASE1),
			}
		};
		let back_fill = vec![Fill::color_tile(self.edge_frame, back_color)];
		let label_fills = string_to_fills(self.label.as_str(), self.edge_frame.move_closer(1), label_color);
		let fills = vec![back_fill, label_fills].into_iter().flatten().collect::<Vec<_>>();
		let captors = {
			let mut event_map = HashMap::new();
			event_map.insert(UserEvent::Select, FabMsg::Press);
			let frame = self.edge_frame;
			vec![Captor { id: captor_id, event_map, frame, pre_focus_msg: FabMsg::Ignore }]
		};
		(fills, captors)
	}


	pub fn min_width_height(&self) -> (u16, u16) {
		let min_cols = self.label.as_str().chars().count().max(3) as u16;
		(min_cols, 1)
	}
}
