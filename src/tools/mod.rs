use crate::components::fill::Fill;
use crate::systems::ViewEffects;
use crate::tools::zrect::ZRect;

pub mod console;
pub mod fill;
pub mod inset;
pub mod sample;
pub mod zrect;

pub trait ViewBuilding {
	type Model: ViewUpdating;

	fn init_view(self, commands: &mut ViewEffects) -> Self::Model;
}

pub trait ViewUpdating {}

pub enum ShapeResult {
	NoChange,
	NewPainters(Vec<Box<dyn Painter + Send + Sync>>),
}

pub trait Shaper {
	fn shape(&mut self, edge_zrect: ZRect) -> ShapeResult;
}

pub trait Painter {
	fn paint(&self) -> Vec<Fill>;
}

pub type BoxRender = Box<dyn Painter + Send + Sync>;
