use crate::resources::solar_dark;
use crate::systems::ViewEffects;
use crate::tools::{BoxPainter, Shaper, ShaperEffects, ShaperMsg, ViewUpdating};
use crate::tools::inset::Inset;
use crate::tools::painters::{BodyPanelPainter, ButtonPainter, ColorIndex, StringPainter, TitlePainter};
use crate::tools::ViewStarting;
use crate::tools::frame::Frame;

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
	edge_frame: Option<Frame>,
}

impl Shaper for MyShaper {
	fn shape(&mut self, msg: ShaperMsg, effects: &mut ShaperEffects) {
		let ShaperMsg::SetEdge(edge_zrect) = msg;
		let edge_frame = edge_zrect.inset(Inset::DoubleCols(1)).move_closer(1);
		if self.edge_frame == Some(edge_frame) {
			return;
		}
		self.edge_frame = Some(edge_frame);
		const EMPTY_TEXT: &str = "Empty in assets";
		const BUTTON_LABEL: &str = "{+}";
		let (title_frame, body_frame) = edge_frame.split_from_top(1);
		let text_frame = body_frame.into_single_row_fixed_width_centered(EMPTY_TEXT.chars().count() as u16).move_closer(1);
		let button_frame = body_frame.into_single_row_fixed_width_at_offset_from_bottom_right(BUTTON_LABEL.chars().count() as u16, 2, 1).move_closer(1);
		let painters: Vec<BoxPainter> = vec![
			Box::new(TitlePainter(title_frame)),
			Box::new(BodyPanelPainter(body_frame)),
			Box::new(StringPainter { frame: text_frame, text: EMPTY_TEXT.to_string(), text_color: ColorIndex(solar_dark::BASE01) }),
			Box::new(ButtonPainter { frame: button_frame, label: BUTTON_LABEL.to_string(), label_color: ColorIndex(solar_dark::BASE01), base_color: ColorIndex(solar_dark::BASE3) }),
		];
		effects.set_painters(painters);
	}
}
