use crate::components::fill::Fill;
use crate::resources::solar_dark;
use crate::tools::fill::Glyph;
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
