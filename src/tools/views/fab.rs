use std::collections::HashMap;
use crate::tools::{Captor, UserEvent};
use crate::tools::frame::Frame;
use crate::tools::views::{ViewChanging, ViewStarting};

pub struct FabInit {
	pub label: String,
}

impl Default for FabInit {
	fn default() -> Self { Self { label: "todo".into() } }
}

impl ViewStarting for FabInit {
	type Model = Fab;

	fn init(self) -> Self::Model {
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
	init: FabInit,
	pressed: bool,
}

impl ViewChanging for Fab {
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
}

impl Fab {
	fn captor(_frame: Frame) -> Captor<FabMsg> {
		let mut event_map = HashMap::new();
		event_map.insert(UserEvent::PressStart, FabMsg::Press);
		event_map.insert(UserEvent::PressEnd, FabMsg::Release);
		Captor { event_map }
	}
}
