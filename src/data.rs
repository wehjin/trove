use std::io;
use std::path::PathBuf;

use echo_lib::{Echo, Point};

pub const ASSET_CUSTODIAN: Point = Point::Static { name: "custodian", aspect: "Asset" };
pub const ASSET_ACCOUNT: Point = Point::Static { name: "account", aspect: "Asset" };
pub const ASSET_SYMBOL: Point = Point::Static { name: "symbol", aspect: "Asset" };
pub const ASSET_SHARES: Point = Point::Static { name: "shares", aspect: "Asset" };
pub const ASSET_CORRAL: Point = Point::Static { name: "corral", aspect: "Asset" };

pub fn echo() -> io::Result<Echo> {
	let folder_path = folder_path()?;
	Ok(Echo::connect(&folder_path))
}

fn folder_path() -> io::Result<PathBuf> {
	let mut path = dirs::home_dir().unwrap();
	path.push(".chad");
	std::fs::create_dir_all(&path)?;
	Ok(path)
}

