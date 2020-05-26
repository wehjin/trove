use std::io;
use std::path::PathBuf;

use echo_lib::{Echo, Object, ObjectFilter, ObjName, Point, Say, Target, Writable};

pub fn echo() -> io::Result<Echo> {
	let folder_path = folder_path()?;
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

fn folder_path() -> io::Result<PathBuf> {
	let mut path = dirs::home_dir().unwrap();
	path.push(".chad");
	std::fs::create_dir_all(&path)?;
	Ok(path)
}

#[derive(Debug, Clone)]
pub struct Lot { object: Object }

impl Lot {
	pub fn symbol(&self) -> &str { self.object.properties[&SYMBOL].as_str() }
	pub fn account(&self) -> &str { self.object.properties[&ACCOUNT].as_str() }
	pub fn custodian(&self) -> &str { self.object.properties[&CUSTODIAN].as_str() }
	pub fn corral(&self) -> &str { self.object.properties[&CORRAL].as_str() }
	pub fn shares(&self) -> u64 { self.object.properties[&SHARES].as_number() }
	pub fn new(symbol: &str, account: &str, custodian: &str, corral: &str, shares: u64) -> Self {
		let object = Object::new(
			&ObjName::String(format!("Lot-{}", rand::random::<usize>())),
			vec![
				(&SYMBOL, Some(Target::Text(symbol.to_string()))),
				(&ACCOUNT, Some(Target::Text(account.to_string()))),
				(&CUSTODIAN, Some(Target::Text(custodian.to_string()))),
				(&CORRAL, Some(Target::Text(corral.to_string()))),
				(&SHARES, Some(Target::Number(shares))),
			],
		);
		Lot { object }
	}
}

impl<'a> ObjectFilter<'a> for Lot {
	fn key_point() -> &'a Point { &SYMBOL }

	fn data_points() -> &'a [&'a Point] {
		&[&SYMBOL, &ACCOUNT, &CUSTODIAN, &SHARES, &CORRAL]
	}

	fn from_name_and_properties(obj_name: &ObjName, attributes: Vec<(&Point, Option<Target>)>) -> Self {
		let object = Object::new(obj_name, attributes);
		Lot { object }
	}
}

impl Writable for Lot {
	fn to_says(&self) -> Vec<Say> { self.object.to_says() }
}

pub const CUSTODIAN: Point = Point::Static { name: "custodian", aspect: "Lot" };
pub const ACCOUNT: Point = Point::Static { name: "account", aspect: "Lot" };
pub const SYMBOL: Point = Point::Static { name: "symbol", aspect: "Lot" };
pub const SHARES: Point = Point::Static { name: "shares", aspect: "Lot" };
pub const CORRAL: Point = Point::Static { name: "corral", aspect: "Lot" };
