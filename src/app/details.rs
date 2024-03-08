use crate::tools::captor::CaptorId;
use crate::tools::fill::{Fill, Glyph};
use crate::tools::frame::Frame;
use crate::tools::frame::layout::Layout;
use crate::tools::inset::Inset;
use crate::tools::solar_dark;
use crate::tools::views::{Shaper, ZMax};

#[derive(Default)]
pub struct Details {
	edge_frame: Frame,
	name_frame: Frame,
	symbol_frame: Frame,
	category_frame: Frame,
}

impl Details {
	pub fn get_fills(&self, _active_captor_id: Option<CaptorId>) -> Vec<Fill> {
		vec![
			Fill { glyph: Glyph::Tile(solar_dark::BASE02), frame: self.edge_frame },
			Fill { glyph: Glyph::Tile(solar_dark::MAGENTA), frame: self.name_frame },
			Fill { glyph: Glyph::Tile(solar_dark::MAGENTA), frame: self.symbol_frame },
			Fill { glyph: Glyph::Tile(solar_dark::MAGENTA), frame: self.category_frame },
		]
	}
}

impl Shaper for Details {
	fn shape(&mut self, edge_frame: Frame) -> ZMax {
		Layout::new(edge_frame)
			.tag(&mut self.edge_frame)
			.inset(Inset::DoubleCols(1))
			.move_closer(1)
			.split_top(1).take(&mut self.name_frame)
			.inset(Inset::Top(1))
			.split_top(1).take(&mut self.symbol_frame)
			.inset(Inset::Top(1))
			.split_top(1).take(&mut self.category_frame)
			.seal()
			.into_z_max()
	}
}

