use std::error::Error;
use std::io::stdout;

use crossterm::{execute, terminal};
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::DisableLineWrap;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnableLineWrap;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;

pub(crate) struct Console;

impl Console {
	pub fn start() -> Result<Self, Box<dyn Error>> {
		let (cols, rows) = terminal::size()?;
		enable_raw_mode()?;
		let msg = format!(" {rows} x {cols} chad! ");
		let upper_half = "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀";
		execute!(
			stdout(),
			EnterAlternateScreen,
			DisableLineWrap,
			SetBackgroundColor(Color::Black),
			Clear(ClearType::All),
			SetForegroundColor(Color::Blue),
			SetBackgroundColor(Color::Black),
			MoveTo(10,2),
			Print(&msg),
			SetForegroundColor(Color::Green),
			SetBackgroundColor(Color::Black),
			MoveTo(10,3 ),
			Print(upper_half),
			MoveTo(10,5 ),
		)?;
		Ok(Self)
	}
	fn exit(&mut self) -> Result<(), Box<dyn Error>> {
		execute!(
			stdout(),
			Clear(ClearType::All),
			LeaveAlternateScreen,
			EnableLineWrap,
		)?;
		disable_raw_mode()?;
		Ok(())
	}
}

impl Drop for Console {
	fn drop(&mut self) {
		self.exit().expect("exit does not fail");
	}
}

