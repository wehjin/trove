use crate::components::fill::Fill;
use crate::components::render::FillFn;
use crate::resources::solar_dark;
use crate::tools::fill;
use crate::tools::fill::Glyph;

pub fn sample_fill_builder() -> Box<FillFn> {
	Box::new(|volume| {
		let (head_volume, body_volume) = volume.split_from_top(1);
		let mut vec = vec![
			Fill { glyph: Glyph::Solid(solar_dark::BASE03), volume: body_volume },
			Fill { glyph: Glyph::Solid(solar_dark::BASE02), volume: head_volume },
		];
		vec.extend(fill::string_to_fills(
			"hello world!",
			head_volume.clone().move_closer(1),
			solar_dark::BASE1,
		));
		vec.extend(fill::string_to_fills(
			"rack the dubs, chad",
			body_volume.clone().move_closer(1),
			solar_dark::BASE0,
		));
		vec
	})
}
