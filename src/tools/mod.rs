use std::collections::HashMap;

use crate::components::fill::Fill;
use crate::tools::frame::Frame;
use crate::view_plugin::tools::ViewEffects;

pub mod console;
pub mod fill;
pub mod frame;
pub mod inset;
pub mod painters;
pub mod sample;
pub mod shapers;
pub mod views;

pub trait ViewStarting {
	type Model: ViewModel + Send + Sync + 'static;

	fn init_view_model(self, effects: &mut ViewEffects<<Self::Model as ViewModel>::Msg>) -> Self::Model;
}

pub trait ViewModel {
	type Msg: Send + Sync + 'static;
	fn update_as_view_model(&mut self, msg: Self::Msg, effects: &mut ViewEffects<Self::Msg>);
}

pub trait Shaper<Msg> {
	fn shape(&mut self, msg: ShaperMsg, effects: &mut ShaperEffects<Msg>);
}

pub type BoxShaper<Msg> = Box<dyn Shaper<Msg> + Send + Sync>;

pub enum ShaperMsg {
	SetEdge(Frame)
}

pub struct ShaperEffects<Msg> {
	pub new_painters: Option<Vec<BoxPainter>>,
	pub new_captor: Option<Captor<Msg>>,
}

impl<Msg> Default for ShaperEffects<Msg> {
	fn default() -> Self {
		Self { new_painters: None, new_captor: None }
	}
}

impl<Msg> ShaperEffects<Msg> {
	pub fn set_painters(&mut self, painters: Vec<BoxPainter>) {
		self.new_painters = Some(painters);
	}
	pub fn set_captor(&mut self, captor: Captor<Msg>) {
		self.new_captor = Some(captor);
	}
}

pub trait Painter {
	fn paint(&self) -> Vec<Fill>;
}

pub type BoxPainter = Box<dyn Painter + Send + Sync>;

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
