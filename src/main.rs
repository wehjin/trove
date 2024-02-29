use std::error::Error;

use crossterm::event::{Event, KeyCode};

use tools::console::Console;
use tools::screen::Screen;

pub mod tools;

fn main() -> Result<(), Box<dyn Error>> {
	let mut console = Console::start()?;
	loop {
		let screen = Screen::new(console.width_height());
		screen.print(&mut console);
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
