use std::error::Error;

use crossterm::event::{Event, KeyCode};

use tools::console::Console;
use tools::screen::Screen;

use crate::tools::sample::SampleApp;
use crate::tools::View;

pub mod tools;

fn main() -> Result<(), Box<dyn Error>> {
	let app = SampleApp;
	let mut console = Console::start()?;
	loop {
		let mut screen = Screen::new(console.width_height());
		screen.add_fills(app.get_fills(screen.to_frame()));
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
