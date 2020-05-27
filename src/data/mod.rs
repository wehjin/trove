use std::io;

use echo_lib::Echo;

pub use self::asset::*;
pub use self::lot::*;

pub mod path;
mod asset;
mod lot;


pub fn echo(folder_name: &str) -> io::Result<Echo> {
	let folder_path = path::echo(folder_name)?;
	let echo = Echo::connect(&folder_path);
	init(&echo)?;
	Ok(echo)
}

fn init(echo: &Echo) -> io::Result<()> {
	let lots = echo.chamber()?.objects::<Lot>()?;
	if lots.len() == 0 {
		let lot = Lot::new("USD", "Main", "Wallet", "Cash", 0);
		echo.write(|write| write.writable(&lot))?
	}
	Ok(())
}
