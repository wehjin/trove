use std::collections::HashMap;

#[allow(unused)]
pub use crossterm::style::Color;

use crate::tools::frame::Frame;

pub mod console;
pub mod fill;
pub mod frame;
pub mod inset;
pub mod sample;
pub mod screen;
pub mod solar_dark;
pub mod views;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum UserEvent {
	Select,
}

pub struct Captor<Msg> {
	pub event_map: HashMap<UserEvent, Msg>,
	pub frame: Frame,
}

impl<Msg> Captor<Msg> {
	pub fn map_msg<WrapMsg>(self, map_msg: impl Fn(Msg) -> WrapMsg) -> Captor<WrapMsg> {
		let Captor { event_map, frame } = self;
		let event_map = event_map.into_iter().map(|(event, msg)| {
			(event, map_msg(msg))
		}).collect::<HashMap<_, _>>();
		Captor { event_map, frame }
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
