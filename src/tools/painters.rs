use crate::components::fill::Fill;
use crate::resources::solar_dark;
use crate::tools::fill::{Glyph, string_to_fills};
use crate::tools::Painter;
use crate::tools::zrect::ZRect;

pub struct BodyPanelPainter(pub ZRect);

impl Painter for BodyPanelPainter {
	fn paint(&self) -> Vec<Fill> {
		let fill = Fill { glyph: Glyph::Solid(solar_dark::BASE03), volume: self.0 };
		vec![fill]
	}
}

pub struct TodoPainter(pub ZRect);

impl Painter for TodoPainter {
	fn paint(&self) -> Vec<Fill> {
		let fill = Fill { glyph: Glyph::Solid(solar_dark::MAGENTA), volume: self.0 };
		vec![fill]
	}
}

pub struct ColorIndex(pub usize);

pub struct ButtonPainter {
	pub zrect: ZRect,
	pub label: String,
	pub label_color: ColorIndex,
	pub base_color: ColorIndex,
}

impl Painter for ButtonPainter {
	fn paint(&self) -> Vec<Fill> {
		let fills1 = string_to_fills(&self.label, self.zrect.move_closer(1), self.label_color.0);
		let fills2 = vec![Fill { glyph: Glyph::Solid(self.base_color.0), volume: self.zrect }];
		vec![fills1, fills2].into_iter().flatten().collect()
	}
}

pub struct StringPainter {
	pub zrect: ZRect,
	pub string: String,
	pub string_color: ColorIndex,
}

impl Painter for StringPainter {
	fn paint(&self) -> Vec<Fill> {
		let zrect = self.zrect;
		string_to_fills(&self.string, zrect, self.string_color.0)
	}
}

pub struct TitlePainter(pub ZRect);

impl Painter for TitlePainter {
	fn paint(&self) -> Vec<Fill> {
		let mut vec = vec![
			Fill { glyph: Glyph::Solid(solar_dark::BASE02), volume: self.0 },
		];
		vec.extend(string_to_fills("hello world!", self.0, solar_dark::BASE1));
		vec
	}
}
