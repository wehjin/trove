#[derive(Copy, Clone)]
pub enum Inset {
	DoubleCols(u8),
	Cols(u8),
	Left(u8),
	Top(u8),
}

impl Inset {
	pub fn to_top_right_bottom_left(&self) -> (u8, u8, u8, u8) {
		match self {
			Inset::DoubleCols(size) => (*size, 2 * size, *size, 2 * size),
			Inset::Cols(size) => (0, *size, 0, *size),
			Inset::Left(cols) => (0, 0, 0, *cols),
			Inset::Top(rows) => (*rows, 0, 0, 0),
		}
	}
}
