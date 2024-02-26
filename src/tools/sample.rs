use crate::components::fill::Fill;
use crate::resources::solar_dark;
use crate::systems::ViewEffects;
use crate::tools::{BoxRender, Painter, Shaper, ShapingResult, ViewUpdating};
use crate::tools::fill::{Glyph, string_to_fills};
use crate::tools::inset::Inset;
use crate::tools::ViewStarting;
use crate::tools::zrect::ZRect;

pub struct SampleAppSettings;

pub struct SampleApp;

impl ViewStarting for SampleAppSettings {
	type Model = SampleApp;
	fn start_view(self, effects: &mut ViewEffects) -> Self::Model {
		effects.set_shaper(MyShaper::default());
		SampleApp
	}
}

impl ViewUpdating for SampleApp {}

#[derive(Default)]
struct MyShaper {
	edge_zrect: Option<ZRect>,
}

impl Shaper for MyShaper {
	fn shape(&mut self, edge_zrect: ZRect) -> ShapingResult {
		if self.edge_zrect == Some(edge_zrect) {
			ShapingResult::NoChange
		} else {
			if self.edge_zrect == Some(edge_zrect) {
				ShapingResult::NoChange
			} else {
				let inset_zrect = edge_zrect.inset(Inset::DoubleCols(1)).move_closer(1);
				let (head_volume, body_volume) = inset_zrect.split_from_top(1);
				let painters: Vec<BoxRender> = vec![
					Box::new(TitleRender(head_volume)),
					Box::new(BodyRender(body_volume)),
				];
				self.edge_zrect = Some(edge_zrect);
				ShapingResult::SetPainters(painters)
			}
		}
	}
}

struct TitleRender(ZRect);

impl Painter for TitleRender {
	fn paint(&self) -> Vec<Fill> {
		let mut vec = vec![
			Fill { glyph: Glyph::Solid(solar_dark::BASE02), volume: self.0 },
		];
		vec.extend(string_to_fills("hello world!", self.0, solar_dark::BASE1));
		vec
	}
}

struct BodyRender(ZRect);

impl Painter for BodyRender {
	fn paint(&self) -> Vec<Fill> {
		let mut vec = vec![
			Fill { glyph: Glyph::Solid(solar_dark::BASE03), volume: self.0 },
		];
		vec.extend(string_to_fills("rack the dubs, chad!", self.0.clone(), solar_dark::BASE0));
		vec
	}
}
