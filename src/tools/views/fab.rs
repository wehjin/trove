use std::thread;
use std::time::Duration;

use crossbeam::channel::Sender;
use rand::random;

use crate::tools::{Cmd, solar_dark};
use crate::tools::beats::{Beat, signal};
use crate::tools::captor::{Captor, CaptorId, CaptorKind};
use crate::tools::fill::{Fill, string_to_fills};
use crate::tools::frame::Frame;
use crate::tools::views::{CursorEvent, Shaping, ZMax};

#[derive(Debug, Copy, Clone)]
pub enum FabMsg {
	Release,
	Ignore,
	ForCursor(CursorEvent),
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
	pub cursor_events_sender: Sender<CursorEvent>,
	pub cursor_events_beat: Beat<FabMsg>,
}

impl Default for Fab {
	fn default() -> Self {
		let (cursor_event_sender, cursor_event_beat) = signal(FabMsg::ForCursor);
		Fab {
			id: random(),
			label: "".to_string(),
			pressed: false,
			edge_frame: Frame::default(),
			cursor_events_sender: cursor_event_sender,
			cursor_events_beat: cursor_event_beat,
		}
	}
}

impl Fab {
	pub fn update(&mut self, msg: FabMsg) -> JustClicked {
		match msg {
			FabMsg::ForCursor(event) => {
				if CursorEvent::Select == event && !self.pressed {
					self.pressed = true;
					JustClicked::No(timer_cmd(100, FabMsg::Release))
				} else {
					JustClicked::No(Cmd::None)
				}
			}
			FabMsg::Release if self.pressed => {
				self.pressed = false;
				JustClicked::Yes
			}
			_ => JustClicked::No(Cmd::None)
		}
	}
	pub fn get_beats(&self) -> Vec<Beat<FabMsg>> {
		vec![self.cursor_events_beat.clone()]
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
			let captor = Captor {
				id: captor_id,
				kind: CaptorKind::default().with_takes_select(),
				cursor_events_sender: Some(self.cursor_events_sender.clone()),
				frame: self.edge_frame,
				pre_focus_msg: FabMsg::Ignore,
			};
			vec![captor]
		};
		(fills, captors)
	}


	pub fn min_width_height(&self) -> (u16, u16) {
		let min_cols = self.label.as_str().chars().count().max(3) as u16;
		(min_cols, 1)
	}
}

impl Shaping for Fab {
	fn shape(&mut self, frame: Frame) -> ZMax {
		self.edge_frame = frame;
		ZMax(frame.z + 5)
	}
}
