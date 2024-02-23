use std::error::Error;
use std::io::stdout;

use crossterm::{ExecutableCommand, execute, terminal};
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, is_raw_mode_enabled, LeaveAlternateScreen};

pub(crate) struct Console {
	restore_cooked_mode: bool,
}

impl Console {
	pub fn start() -> Result<Self, Box<dyn Error>> {
		let restore_cooked_mode = !is_raw_mode_enabled()?;
		let (cols, rows) = terminal::size()?;
		enable_raw_mode()?;
		let msg = format!("{rows} x {cols} chad!");
		execute!(
			stdout(),
			EnterAlternateScreen,
			MoveTo(5,5),
			SetForegroundColor(Color::Yellow),
			SetBackgroundColor(Color::Black),
			Clear(ClearType::All),
			Print(&msg),
		)?;
		Ok(Self { restore_cooked_mode })
	}
	fn exit(&mut self) -> Result<(), Box<dyn Error>> {
		if self.restore_cooked_mode {
			disable_raw_mode()?;
		}
		stdout().execute(LeaveAlternateScreen)?;
		Ok(())
	}
}

impl Drop for Console {
	fn drop(&mut self) {
		self.exit().expect("exit");
	}
}

