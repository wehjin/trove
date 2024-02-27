use crate::components::fill::Fill;
use crate::resources::solar_dark;
use crate::systems::ViewEffects;
use crate::tools::{BoxRender, Painter, Shaper, ShapingResult, ViewUpdating};
use crate::tools::fill::{Glyph, string_to_fills};
use crate::tools::inset::Inset;
use crate::tools::painters::BodyPanelPainter;
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
		let edge_zrect = edge_zrect.inset(Inset::DoubleCols(1)).move_closer(1);
		if self.edge_zrect == Some(edge_zrect) {
			ShapingResult::NoChange
		} else {
			self.edge_zrect = Some(edge_zrect);
			let (head_zrect, body_zrect) = edge_zrect.split_from_top(1);
			let message = "Empty in assets".to_string();
			let message_zrect = body_zrect.into_single_row_fixed_width_centered(message.chars().count() as u16).move_closer(1);
			let button = "{+}".to_string();
			let button_zrect = body_zrect.into_single_row_fixed_width_at_offset_from_bottom_right(button.chars().count() as u16, 2, 1);
			let painters: Vec<BoxRender> = vec![
				Box::new(TitlePainter(head_zrect)),
				Box::new(BodyPanelPainter(body_zrect)),
				Box::new(StringPainter {
					zrect: message_zrect,
					string: message,
					string_color: ColorIndex(solar_dark::BASE01),
				}),
				Box::new(ButtonPainter {
					zrect: button_zrect,
					label: button,
					label_color: ColorIndex(solar_dark::BASE01),
					base_color: ColorIndex(solar_dark::BASE3),
				}),
			];
			ShapingResult::SetPainters(painters)
		}
	}
}

pub struct ColorIndex(pub usize);

pub struct ButtonPainter {
	zrect: ZRect,
	label: String,
	label_color: ColorIndex,
	base_color: ColorIndex,
}

impl Painter for ButtonPainter {
	fn paint(&self) -> Vec<Fill> {
		let fills1 = string_to_fills(&self.label, self.zrect.move_closer(1), self.label_color.0);
		let fills2 = vec![Fill { glyph: Glyph::Solid(self.base_color.0), volume: self.zrect }];
		vec![fills1, fills2].into_iter().flatten().collect()
	}
}

pub struct StringPainter {
	zrect: ZRect,
	string: String,
	string_color: ColorIndex,
}

impl Painter for StringPainter {
	fn paint(&self) -> Vec<Fill> {
		let zrect = self.zrect;
		string_to_fills(&self.string, zrect, self.string_color.0)
	}
}

struct TitlePainter(ZRect);

impl Painter for TitlePainter {
	fn paint(&self) -> Vec<Fill> {
		let mut vec = vec![
			Fill { glyph: Glyph::Solid(solar_dark::BASE02), volume: self.0 },
		];
		vec.extend(string_to_fills("hello world!", self.0, solar_dark::BASE1));
		vec
	}
}
