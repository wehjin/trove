use std::error::Error;
use std::fmt::Display;
use std::io::{stdout, Write};

use crossterm::{execute, queue, terminal};
use crossterm::cursor::{Hide, MoveTo, Show};
pub use crossterm::event::Event as ConsoleEvent;
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, DisableLineWrap, enable_raw_mode, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen};

pub struct Console;

impl Drop for Console {
	fn drop(&mut self) {
		execute!(
			stdout(),
			Clear(ClearType::All),
			Show,
			LeaveAlternateScreen,
			EnableLineWrap,
		).expect("execute");
		disable_raw_mode().expect("disable_raw_mode");
	}
}

impl Console {
	pub fn start() -> Result<Self, Box<dyn Error>> {
		enable_raw_mode()?;
		execute!(
			stdout(),
			EnterAlternateScreen,
			DisableLineWrap,
			SetBackgroundColor(Color::Black),
			Hide,
			Clear(ClearType::All),
		)?;
		Ok(Self)
	}
	pub fn read(&self) -> std::io::Result<ConsoleEvent> {
		crossterm::event::read()
	}
	pub fn width_height(&self) -> (u16, u16) {
		terminal::size().expect("size")
	}

	pub fn move_color_print<T: Display>(&self, col: u16, row: u16, text: T, fg_color: Color, bg_color: Color) {
		queue!(
			stdout(),
			MoveTo(col, row),
			SetForegroundColor(fg_color),
			SetBackgroundColor(bg_color),
			Print(text),
		).expect("moveto, set_x_color, print");
	}
	pub fn flush(&mut self) {
		queue!(
			stdout(),
			MoveTo(0,0)
		).expect("move-to 0,0");
		stdout().flush().expect("flush");
	}
	pub fn _print(&self, msg: &str) {
		let (cols, rows) = terminal::size().expect("size");
		let title = format!(" {rows} x {cols} {msg} ");
		let underline = (0..title.len()).map(|_| '▀').collect::<String>();
		queue!(
			stdout(),
			SetForegroundColor(Color::Blue),
			SetBackgroundColor(Color::Black),
			MoveTo(10,2),
			Print(&title),
			SetForegroundColor(Color::Green),
			SetBackgroundColor(Color::Black),
			MoveTo(10,3 ),
			Print(underline),
			MoveTo(10,5 ),
		).expect("execute");
	}
}
