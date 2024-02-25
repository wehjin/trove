use crate::components::fill::Fill;
use crate::resources::solar_dark;
use crate::tools::{Layout, Render, RenderBox, ViewModel};
use crate::tools::fill::{Glyph, string_to_fills};
use crate::tools::inset::Inset;
use crate::tools::volume::ZRect;

pub struct SampleApp;

impl ViewModel<()> for SampleApp {
	fn to_layout(&self) -> Box<dyn Layout + Send + Sync> {
		Box::new(MyLayout {})
	}
}

struct MyLayout {}

impl Layout for MyLayout {
	fn run_layout(&self, volume: ZRect) -> Vec<RenderBox> {
		let volume = volume.inset(Inset::DoubleCols(1)).move_closer(1);
		let (head_volume, body_volume) = volume.split_from_top(1);
		vec![
			Box::new(TitleRender(head_volume)),
			Box::new(BodyRender(body_volume)),
		]
	}
}

struct TitleRender(ZRect);

impl Render for TitleRender {
	fn run_render(&self) -> Vec<Fill> {
		let mut vec = vec![
			Fill { glyph: Glyph::Solid(solar_dark::BASE02), volume: self.0 },
		];
		vec.extend(string_to_fills("hello world!", self.0, solar_dark::BASE1));
		vec
	}
}

struct BodyRender(ZRect);

impl Render for BodyRender {
	fn run_render(&self) -> Vec<Fill> {
		let mut vec = vec![
			Fill { glyph: Glyph::Solid(solar_dark::BASE03), volume: self.0 },
		];
		vec.extend(string_to_fills("rack the dubs, chad!", self.0.clone(), solar_dark::BASE0));
		vec
	}
}
