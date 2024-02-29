use crossterm::style::Color;
use crate::tools::console::Console;

pub struct Screen {
	max_col: u16,
	max_row: u16,
}

impl Screen {
	pub fn new((cols, rows): (u16, u16)) -> Self {
		Self { max_col: cols, max_row: rows }
	}
	pub fn print(&self, console: &mut Console) {
		for row in 0..self.max_row {
			for col in 0..self.max_col {
				console.move_color_print(col, row, 'k', Color::Yellow, Color::Black)
			}
		}
		console.flush();
	}
}
