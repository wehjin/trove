use std::error::Error;

use crossterm::event::{Event, KeyCode};

use tools::console::Console;
use tools::screen::Screen;
use tools::views::View;

use crate::tools::sample::SampleAppInit;
use crate::tools::views::ViewStarting;

pub mod tools;

fn main() -> Result<(), Box<dyn Error>> {
	let app = SampleAppInit.into_view();
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
