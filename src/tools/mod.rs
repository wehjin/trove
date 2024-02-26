use crate::components::fill::Fill;
use crate::systems::ViewEffects;
use crate::tools::zrect::ZRect;

pub mod console;
pub mod fill;
pub mod inset;
pub mod sample;
pub mod zrect;

pub trait ViewStarting {
	type Model: ViewUpdating;

	fn start_view(self, commands: &mut ViewEffects) -> Self::Model;
}

pub trait ViewUpdating {}

pub enum ShapingResult {
	NoChange,
	SetPainters(Vec<Box<dyn Painter + Send + Sync>>),
}

pub trait Shaper {
	fn shape(&mut self, edge_zrect: ZRect) -> ShapingResult;
}

pub trait Painter {
	fn paint(&self) -> Vec<Fill>;
}

pub type BoxRender = Box<dyn Painter + Send + Sync>;
