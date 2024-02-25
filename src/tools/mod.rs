use crate::components::fill::Fill;
use crate::resources::solar_dark;
use crate::tools::fill::{Glyph, string_to_fills};
use crate::tools::inset::Inset;
use crate::tools::volume::Volume;

pub mod console;
pub mod fill;
pub mod inset;
pub mod render;
pub mod volume;

pub trait ViewModel<ViewMsg> {
	fn to_layouts(&self) -> Vec<Box<LayoutFn>>;
}

// TODO Clean this up, too hard to use.  Maybe LayoutRenderer becomes Renderer?
pub type LayoutFn = dyn Fn(Volume) -> Vec<Box<RenderFn>> + Send + Sync;
pub type RenderFn = dyn Fn() -> Vec<Fill> + Send + Sync;

pub struct SampleApp;

impl ViewModel<()> for SampleApp {
	fn to_layouts(&self) -> Vec<Box<LayoutFn>> {
		vec![Box::new(Self::layout)]
	}
}

impl SampleApp {
	fn layout(volume: Volume) -> Vec<Box<RenderFn>> {
		let inset_volume = volume.inset(Inset::DoubleCols(1));
		let renderer: Box<RenderFn> = Box::new(move || Self::render(inset_volume));
		vec![renderer]
	}

	fn render(volume: Volume) -> Vec<Fill> {
		let (head_volume, body_volume) = volume.split_from_top(1);
		let mut vec = vec![
			Fill { glyph: Glyph::Solid(solar_dark::BASE03), volume: body_volume },
			Fill { glyph: Glyph::Solid(solar_dark::BASE02), volume: head_volume },
		];
		vec.extend(string_to_fills("hello world!", head_volume.clone().move_closer(1), solar_dark::BASE1));
		vec.extend(string_to_fills("rack the dubs, chad", body_volume.clone().move_closer(1), solar_dark::BASE0));
		vec
	}
}
