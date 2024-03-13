use crate::data::Asset;
use crate::tools::captor::{Captor, CaptorId};
use crate::tools::fill::{Fill, Glyph, string_to_fills};
use crate::tools::frame::Frame;
use crate::tools::frame::layout::Layout;
use crate::tools::inset::Inset;
use crate::tools::solar_dark;
use crate::tools::views::{Shaping, Updating, Viewing, ZMax};
use crate::tools::views::text_edit::{TextEditor, TextEditorMsg};

#[derive(Default)]
pub struct Details {
	edge_frame: Frame,
	name_frame: Frame,
	symbol_frame: Frame,
	category_frame: Frame,
	asset: Option<Asset>,
	text_edit: TextEditor,
}

impl Viewing for Details {
	type Msg = DetailsMsg;

	fn get_fills_captors(&self, active_captor_id: Option<CaptorId>) -> (Vec<Fill>, Vec<Captor<Self::Msg>>) {
		let back_fill = Fill { glyph: Glyph::Tile(solar_dark::BASE02), frame: self.edge_frame };
		if let Some(asset) = &self.asset {
			let back_fills = vec![back_fill];
			let name_fills = string_to_fills(asset.name.as_str(), self.name_frame, solar_dark::BASE2);
			let category_fills = string_to_fills(asset.kind.as_str(), self.category_frame, solar_dark::BASE00);
			let symbol_fills = string_to_fills(asset.symbol.as_str(), self.symbol_frame, solar_dark::BASE1);
			let (edit_fills, edit_captors) = self.text_edit.get_fills_captors(active_captor_id);
			let fills = vec![
				back_fills,
				name_fills,
				symbol_fills,
				category_fills,
				edit_fills,
			].into_iter().flatten().collect();
			let captors = edit_captors.into_iter()
				.map(|it| it.map_msg(DetailsMsg::ForTextEditor))
				.collect::<Vec<_>>();
			(fills, captors)
		} else {
			let fills = vec![
				back_fill,
				Fill { glyph: Glyph::Tile(solar_dark::MAGENTA), frame: self.name_frame },
				Fill { glyph: Glyph::Tile(solar_dark::MAGENTA), frame: self.category_frame },
				Fill { glyph: Glyph::Tile(solar_dark::MAGENTA), frame: self.symbol_frame },
			];
			(fills, vec![])
		}
	}
}

#[derive(Debug, Clone)]
pub enum DetailsMsg {
	SetAsset(Asset),
	ForTextEditor(TextEditorMsg),
}

impl Updating for Details {
	type Msg = DetailsMsg;

	fn update(&mut self, msg: Self::Msg) {
		match msg {
			DetailsMsg::SetAsset(asset) => self.asset = Some(asset),
			DetailsMsg::ForTextEditor(msg) => self.text_edit.update(msg),
		}
	}
}

impl Shaping for Details {
	fn shape(&mut self, edge_frame: Frame) -> ZMax {
		Layout::new(edge_frame)
			.tag(&mut self.edge_frame)
			.inset(Inset::DoubleCols(1))
			.move_closer(1)
			.split_top(1).take(&mut self.name_frame)
			.split_top(1).take(&mut self.category_frame)
			.inset(Inset::Top(1))
			.split_top(1).take(&mut self.symbol_frame)
			.inset(Inset::Top(1))
			.split_top(1).shape(&mut self.text_edit).seal()
			.seal()
			.into_z_max()
	}
}

