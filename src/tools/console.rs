use std::error::Error;
use std::io::{stdout, Write};

use bevy::prelude::Resource;
use crossterm::{queue, terminal};
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};

use crate::components::console::Position;

#[derive(Resource)]
pub struct Console;

impl Console {
	pub fn width_height(&self) -> (u16, u16) {
		terminal::size().expect("size")
	}
	pub fn move_print(&self, col: u16, row: u16, msg: &str) {
		queue!(
			stdout(),
			MoveTo(col, row),
			SetForegroundColor(Color::Yellow),
			SetBackgroundColor(Color::Black),
			Print(msg),
		).expect("moveto, print");
	}
	pub fn color(&self, pos: &Position, color: Color) {
		let width = pos.right - pos.left;
		let spaces = (0..width).map(|_| ' ').collect::<String>();
		let col = pos.left;
		for row in pos.top..pos.bottom {
			queue!(
				stdout(),
				MoveTo(col, row),
				SetBackgroundColor(color),
				Print(&spaces),
			).expect("moveto, print");
		}
	}
	pub fn flush(&mut self) {
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
	pub fn start() -> Result<Self, Box<dyn Error>> {
		// For ow we are using the console only for discovering the size
		// and do not need to change modes.
		//
		// enable_raw_mode()?;
		// execute!(
		// 	stdout(),
		// 	EnterAlternateScreen,
		// 	DisableLineWrap,
		// 	SetBackgroundColor(Color::Black),
		// 	Clear(ClearType::All),
		// )?;
		Ok(Self)
	}
	pub fn stop() -> Result<(), Box<dyn Error>> {
		// execute!(
		// 	stdout(),
		// 	Clear(ClearType::All),
		// 	LeaveAlternateScreen,
		// 	EnableLineWrap,
		// )?;
		//disable_raw_mode()?;
		Ok(())
	}
}
