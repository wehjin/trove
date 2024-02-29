use std::collections::HashMap;

#[allow(unused)]
pub use crossterm::style::Color;

use fill::Fill;

use crate::tools::frame::Frame;

pub mod console;
pub mod fill;
pub mod frame;
pub mod inset;
pub mod painters;
pub mod sample;
pub mod screen;
pub mod shapers;
pub mod solar_dark;
pub mod views;

pub trait ViewStarting {
	type Model: ViewChanging;

	fn init(self) -> Self::Model;
}

pub trait ViewChanging {
	type Msg: Send + Sync + 'static;
	fn update(&mut self, msg: Self::Msg);
}

pub trait View {
	fn get_fills(&self, edge_frame: Frame) -> Vec<Fill>;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum UserEvent {
	PressStart,
	PressEnd,
}

pub struct Captor<Msg> {
	pub event_map: HashMap<UserEvent, Msg>,
}

impl<Msg: Copy> Captor<Msg> {
	pub fn get_msg(&self, key: &UserEvent) -> Option<Msg> {
		if let Some(msg) = self.event_map.get(key) {
			Some(*msg)
		} else {
			None
		}
	}
}
