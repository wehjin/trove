use std::collections::HashMap;

use crate::tools::frame::Frame;
use crate::tools::UserEvent;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CaptorId(pub u64, pub usize);

pub struct Captor<Msg> {
	pub id: CaptorId,
	pub event_map: HashMap<UserEvent, Msg>,
	pub frame: Frame,
}

impl<Msg> Captor<Msg> {
	pub fn map_msg<WrapMsg>(self, map_msg: impl Fn(Msg) -> WrapMsg) -> Captor<WrapMsg> {
		let Captor { id, event_map, frame } = self;
		let event_map = event_map.into_iter().map(|(event, msg)| {
			(event, map_msg(msg))
		}).collect::<HashMap<_, _>>();
		Captor { id, event_map, frame }
	}
}

impl<Msg: Copy> Captor<Msg> {
	pub fn get_msg(&self, key: UserEvent) -> Option<Msg> {
		if let Some(msg) = self.event_map.get(&key) {
			Some(*msg)
		} else {
			None
		}
	}
}
