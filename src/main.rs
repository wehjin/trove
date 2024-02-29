use std::error::Error;

use crossterm::event::{Event, KeyCode};

use tools::console::Console;
use tools::screen::Screen;

use crate::tools::fill::{Fill, Glyph};
use crate::tools::frame::Frame;
use crate::tools::solar_dark;

pub mod tools;

fn main() -> Result<(), Box<dyn Error>> {
	let mut console = Console::start()?;
	loop {
		let mut screen = Screen::new(console.width_height());
		screen.add_fills(vec![
			Fill {
				glyph: Glyph::Rune('k', solar_dark::BASE1),
				frame: Frame::from_cols_rows_z(5, 1, 1),
			},
			Fill {
				glyph: Glyph::Tile(solar_dark::BASE02),
				frame: Frame::from_cols_rows_z(screen.cols(), 1, 0),
			},
			Fill {
				glyph: Glyph::Rune('z', solar_dark::BASE0),
				frame: Frame::from_cols_rows_z(10, 1, 1).move_down(1),
			},
			Fill {
				glyph: Glyph::Tile(solar_dark::BASE03),
				frame: Frame::from_cols_rows_z(screen.cols(), screen.rows() - 1, 0).move_down(1),
			},
		]);
		screen.print_to(&mut console);
		match console.read()? {
			Event::FocusGained => {}
			Event::FocusLost => {}
			Event::Key(key_event) => {
				if key_event.code == KeyCode::Char('q') {
					break;
				}
			}
			Event::Mouse(_) => {}
			Event::Paste(_) => {}
			Event::Resize(_, _) => {}
		}
	}
	drop(console);
	Ok(())
}
