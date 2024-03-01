use std::error::Error;

use crossterm::event::{Event, KeyCode};

use tools::console::Console;
use tools::screen::Screen;
use tools::views::View;

use crate::tools::sample::SampleAppInit;
use crate::tools::UserEvent;
use crate::tools::views::ViewStarting;

pub mod tools;

fn main() -> Result<(), Box<dyn Error>> {
	let mut app = SampleAppInit.into_view();
	let mut console = Console::start()?;
	loop {
		let mut screen = Screen::new(console.width_height());
		let (fills, captors) = app.get_fills_captors(screen.to_frame());
		screen.add_fills(fills);
		screen.print_to(&mut console);
		match console.read()? {
			Event::FocusGained => {}
			Event::FocusLost => {}
			Event::Key(key_event) => {
				if key_event.code == KeyCode::Char('q') {
					break;
				}
				let user_event = if key_event.code == KeyCode::Char(' ') {
					Some(UserEvent::Select)
				} else {
					None
				};
				if let Some(user_event) = user_event {
					// TODO Impl better policy. Rudimentary policy works only for this specific case.
					for captor in &captors {
						if let Some(msg) = captor.get_msg(user_event) {
							app.update(msg);
							break;
						}
					}
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
