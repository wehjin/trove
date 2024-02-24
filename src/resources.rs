use bevy::prelude::{Color, Resource};

#[derive(Resource)]
pub struct Palette16 {
	rgb_u8s: [[u8; 3]; 16],
}

impl Palette16 {
	pub const SOLAR_DARK: Palette16 = Palette16 { rgb_u8s: SOLAR_DARK_RGB };

	pub fn to_colors(&self) -> Vec<Color> {
		self.rgb_u8s.iter().map(|rgb| Color::rgb_u8(rgb[0], rgb[1], rgb[2])).collect()
	}
}

const SOLAR_DARK_RGB: [[u8; 3]; 16] = [
	[0, 43, 54],        // base03
	[7, 54, 66],        // base02
	[88, 110, 117],     // base01
	[101, 123, 131],    // base00
	[131, 148, 150],    // base0
	[147, 161, 161],    // base1
	[238, 232, 213],    // base2
	[253, 246, 227],    // base3
	[181, 137, 0],      // yellow
	[203, 75, 22],      // orange
	[220, 50, 47],      // red
	[211, 54, 130],     // magenta
	[108, 113, 196],    // violet
	[38, 139, 210],     // blue
	[42, 161, 152],     // cyan
	[133, 153, 0],      // green
];
