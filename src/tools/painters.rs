use crate::components::fill::Fill;
use crate::resources::solar_dark;
use crate::tools::fill::{Glyph, string_to_fills};
use crate::tools::frame::Frame;
use crate::tools::Painter;

pub struct BodyPanelPainter(pub Frame);

impl Painter for BodyPanelPainter {
	fn paint(&self) -> Vec<Fill> {
		let fill = Fill { glyph: Glyph::Solid(solar_dark::BASE03), volume: self.0 };
		vec![fill]
	}
}

pub struct TodoPainter(pub Frame);

impl Painter for TodoPainter {
	fn paint(&self) -> Vec<Fill> {
		let fill = Fill { glyph: Glyph::Solid(solar_dark::MAGENTA), volume: self.0 };
		vec![fill]
	}
}

pub struct ColorIndex(pub usize);

pub struct ButtonPainter {
	pub frame: Frame,
	pub label: String,
	pub label_color: ColorIndex,
	pub base_color: ColorIndex,
}

impl Painter for ButtonPainter {
	fn paint(&self) -> Vec<Fill> {
		let frame = self.frame
			.into_single_row_fixed_width_centered(self.label.chars().count() as u16);
		let fills1 = string_to_fills(&self.label, frame.move_closer(1), self.label_color.0);
		let fills2 = vec![Fill { glyph: Glyph::Solid(self.base_color.0), volume: frame }];
		vec![fills1, fills2].into_iter().flatten().collect()
	}
}

pub struct StringPainter {
	pub frame: Frame,
	pub text: String,
	pub text_color: ColorIndex,
}

impl Painter for StringPainter {
	fn paint(&self) -> Vec<Fill> {
		let zrect = self.frame;
		string_to_fills(&self.text, zrect, self.text_color.0)
	}
}

pub struct TitlePainter(pub Frame);

impl Painter for TitlePainter {
	fn paint(&self) -> Vec<Fill> {
		let mut vec = vec![
			Fill { glyph: Glyph::Solid(solar_dark::BASE02), volume: self.0 },
		];
		vec.extend(string_to_fills("hello world!", self.0, solar_dark::BASE1));
		vec
	}
}
