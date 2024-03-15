use crossbeam::channel::Sender;

use crate::tools::frame::Frame;
use crate::tools::views::CursorEvent;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CaptorId(pub u64, pub usize);

#[derive(Debug, Clone)]
pub struct Captor<Msg> {
	pub id: CaptorId,
	pub kind: CaptorKind,
	pub cursor_events_sender: Option<Sender<CursorEvent>>,
	pub frame: Frame,
	pub pre_focus_msg: Msg,
}

impl<Msg: Clone> Captor<Msg> {
	pub fn get_focus_msg(&self) -> Option<Msg> {
		Some(self.pre_focus_msg.clone())
	}
}

impl<Msg> Captor<Msg> {
	pub fn map_msg<WrapMsg>(self, map_msg: impl Fn(Msg) -> WrapMsg) -> Captor<WrapMsg> {
		Captor {
			id: self.id,
			kind: self.kind,
			cursor_events_sender: self.cursor_events_sender,
			frame: self.frame,
			pre_focus_msg: map_msg(self.pre_focus_msg),
		}
	}
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct CaptorKind {
	pub takes_chars: bool,
	pub takes_delete_back: bool,
	pub takes_select: bool,
}

impl CaptorKind {
	pub fn with_takes_select(mut self) -> Self {
		self.takes_select = true;
		self
	}
}