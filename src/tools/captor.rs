use std::collections::HashMap;

use crate::tools::frame::Frame;
use crate::tools::UserEvent;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CaptorId(pub u64, pub usize);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Captor<Msg> {
	pub id: CaptorId,
	pub event_map: HashMap<UserEvent, Msg>,
	pub frame: Frame,
	pub pre_focus_msg: Msg,
}

impl<Msg: Clone> Captor<Msg> {
	pub fn get_msg(&self, key: UserEvent) -> Option<Msg> {
		if let Some(msg) = self.event_map.get(&key) {
			Some(msg.clone())
		} else {
			None
		}
	}
	pub fn get_focus_msg(&self) -> Option<Msg> {
		Some(self.pre_focus_msg.clone())
	}
}

impl<Msg> Captor<Msg> {
	pub fn map_msg<WrapMsg>(self, map_msg: impl Fn(Msg) -> WrapMsg) -> Captor<WrapMsg> {
		Captor {
			id: self.id,
			event_map: self.event_map
				.into_iter()
				.map(|(event, msg)| (event, map_msg(msg)))
				.collect(),
			frame: self.frame,
			pre_focus_msg: map_msg(self.pre_focus_msg),
		}
	}
}
