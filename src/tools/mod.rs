use crate::components::fill::Fill;
use crate::systems::{ViewEffects};
use crate::tools::frame::Frame;

pub mod console;
pub mod fill;
pub mod frame;
pub mod inset;
pub mod painters;
pub mod sample;
pub mod views;

pub trait ViewStarting {
	type Model: ViewUpdating;

	fn start_view(self, effects: &mut ViewEffects) -> Self::Model;
}

pub trait ViewUpdating {}

pub trait Shaper {
	fn shape(&mut self, msg: ShaperMsg, effects: &mut ShaperEffects);
}

pub enum ShaperMsg {
	SetEdge(Frame)
}

#[derive(Default)]
pub struct ShaperEffects {
	pub new_painters: Option<Vec<BoxPainter>>,
}

impl ShaperEffects {
	pub fn set_painters(&mut self, painters: Vec<BoxPainter>) {
		self.new_painters = Some(painters);
	}
}

pub trait Painter {
	fn paint(&self) -> Vec<Fill>;
}


pub type BoxPainter = Box<dyn Painter + Send + Sync>;
