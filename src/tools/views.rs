use std::collections::HashMap;

use crate::tools::{BoxPainter, Captor, Shaper, solar_dark, UserEvent, ViewModel, ViewStarting};
use crate::tools::frame::Frame;
use crate::tools::painters::{ButtonPainter, ColorIndex};
use crate::tools::shapers::EdgeShaper;

pub struct FabInit {
	pub label: String,
}

impl Default for FabInit {
	fn default() -> Self { Self { label: "todo".into() } }
}

impl ViewStarting for FabInit {
	type Model = Fab;

	fn init_view_model(self) -> Self::Model {
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

impl ViewModel for Fab {
	type Msg = FabMsg;

	fn update_as_view_model(&mut self, msg: Self::Msg) {
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
	fn to_shaper(&self) -> impl Shaper<FabMsg> + Send + Sync + 'static {
		let (base_index, label_index) = match self.pressed {
			false => (solar_dark::BASE3, solar_dark::BASE00),
			true => (solar_dark::BASE1, solar_dark::BASE02),
		};
		let shaper = EdgeShaper::new(
			{
				let model_label: String = self.init.label.to_string();
				move |frame| {
					Box::new(ButtonPainter {
						frame,
						label: model_label.to_string(),
						label_color: ColorIndex(label_index),
						base_color: ColorIndex(base_index),
					}) as BoxPainter
				}
			},
			Self::captor);
		shaper
	}

	fn captor(_frame: Frame) -> Captor<FabMsg> {
		let mut event_map = HashMap::new();
		event_map.insert(UserEvent::PressStart, FabMsg::Press);
		event_map.insert(UserEvent::PressEnd, FabMsg::Release);
		Captor { event_map }
	}
}
