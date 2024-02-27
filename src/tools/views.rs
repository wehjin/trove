use bevy::utils::default;

use crate::resources::solar_dark;
use crate::systems::ViewEffects;
use crate::tools::{BoxPainter, ViewStarting, ViewUpdating};
use crate::tools::painters::{ButtonPainter, ColorIndex};
use crate::tools::shapers::PainterShaper;

pub struct FabInit {
	pub label: String,
}

impl Default for FabInit {
	fn default() -> Self { Self { label: "todo".into() } }
}

impl ViewStarting for FabInit {
	type Model = Fab;

	fn start_view(self, effects: &mut ViewEffects) -> Self::Model {
		let model = Fab { init: self, ..default() };
		let (base_index, label_index) = match model.pressed {
			false => (solar_dark::BASE3, solar_dark::BASE00),
			true => (solar_dark::BASE2, solar_dark::BASE01),
		};
		let shaper = PainterShaper::new({
			let model_label: String = model.init.label.to_string();
			move |frame| {
				Box::new(ButtonPainter {
					frame,
					label: model_label.to_string(),
					label_color: ColorIndex(label_index),
					base_color: ColorIndex(base_index),
				}) as BoxPainter
			}
		});
		effects.set_shaper(shaper);
		model
	}
}

#[derive(Default)]
pub struct Fab {
	init: FabInit,
	pressed: bool,
}

impl ViewUpdating for Fab {}
