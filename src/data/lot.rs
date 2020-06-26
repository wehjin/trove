use echo_lib::{Object, ObjectFilter, ObjectId, Point, Say, Target, Writable};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lot { object: Object }

impl Lot {
	pub fn symbol(&self) -> &str { self.object.properties[&SYMBOL].as_str() }
	pub fn account(&self) -> &str { self.object.properties[&ACCOUNT].as_str() }
	pub fn custodian(&self) -> &str { self.object.properties[&CUSTODIAN].as_str() }
	pub fn corral(&self) -> &str { self.object.properties[&CORRAL].as_str() }
	pub fn shares(&self) -> u64 { self.object.properties[&SHARES].as_number() }
	pub fn new(symbol: &str, account: &str, custodian: &str, corral: &str, shares: u64) -> Self {
		let object = Object::new(
			&ObjectId::String(format!("Lot-{}", rand::random::<usize>())),
			vec![
				(&SYMBOL, Some(Target::String(symbol.to_string()))),
				(&ACCOUNT, Some(Target::String(account.to_string()))),
				(&CUSTODIAN, Some(Target::String(custodian.to_string()))),
				(&CORRAL, Some(Target::String(corral.to_string()))),
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

	fn from_name_and_properties(obj_name: &ObjectId, attributes: Vec<(&Point, Option<Target>)>) -> Self {
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
