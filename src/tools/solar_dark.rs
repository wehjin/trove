use crate::tools::Color;

pub fn color_by_index(color: usize) -> Color { Color::AnsiValue(ANSI[color]) }

const ANSI: [u8; 16] = [
	234,
	235,
	240,
	241,
	244,
	245,
	254,
	230,
	136,
	166,
	160,
	125,
	61,
	33,
	37,
	64,
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