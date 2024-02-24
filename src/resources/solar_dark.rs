use crate::resources::Palette16;

pub const PALETTE16: Palette16 = Palette16 { rgb_u8s: RGB };

const RGB: [[u8; 3]; 16] = [
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

#[allow(unused)]
pub const BASE03: usize = 0;
#[allow(unused)]
pub const BASE02: usize = 1;
#[allow(unused)]
pub const BASE01: usize = 2;
#[allow(unused)]
pub const BASE00: usize = 3;
#[allow(unused)]
pub const BASE0: usize = 4;
#[allow(unused)]
pub const BASE1: usize = 5;
#[allow(unused)]
pub const BASE2: usize = 6;
#[allow(unused)]
pub const BASE3: usize = 7;
#[allow(unused)]
pub const YELLOW: usize = 8;
#[allow(unused)]
pub const ORANGE: usize = 9;
#[allow(unused)]
pub const RED: usize = 10;
#[allow(unused)]
pub const MAGENTA: usize = 11;
#[allow(unused)]
pub const VIOLET: usize = 12;
#[allow(unused)]
pub const BLUE: usize = 13;
#[allow(unused)]
pub const CYAN: usize = 14;
#[allow(unused)]
pub const GREEN: usize = 15;
