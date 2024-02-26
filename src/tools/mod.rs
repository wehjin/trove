use crate::components::fill::Fill;
use crate::tools::zrect::ZRect;

pub mod console;
pub mod fill;
pub mod inset;
pub mod sample;
pub mod zrect;

pub trait ShapePaint {
	fn to_shaper(&self) -> Box<dyn Shaper + Send + Sync>;
}

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
