use bevy::prelude::{Color, Resource};

#[derive(Resource)]
pub struct Palette16 {
	rgb_u8s: [[u8; 3]; 16],
}

impl Palette16 {
	pub fn to_colors(&self) -> Vec<Color> {
		self.rgb_u8s.iter().map(|rgb| Color::rgb_u8(rgb[0], rgb[1], rgb[2])).collect()
	}
}

pub mod solar_dark;