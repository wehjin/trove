use crate::components::fill::Fill;
use crate::tools::volume::ZRect;

pub mod console;
pub mod fill;
pub mod inset;
pub mod layouts;
pub mod renders;
pub mod sample;
pub mod volume;

pub trait ViewModel<ViewMsg> {
	fn to_layout(&self) -> Box<dyn Layout + Send + Sync>;
}

pub trait Layout {
	fn run_layout(&self, volume: ZRect) -> Vec<Box<dyn Render + Send + Sync>>;
}

pub trait Render {
	fn run_render(&self) -> Vec<Fill>;
}

pub type RenderBox = Box<dyn Render + Send + Sync>;