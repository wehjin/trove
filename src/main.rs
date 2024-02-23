use std::error::Error;

use crossterm::event::read;
use crossterm::ExecutableCommand;

use console::Console;

fn main() -> Result<(), Box<dyn Error>> {
	let _terminal = Console::start()?;
	read()?;
	Ok(())
}

mod console;
