use echo_lib::Echo;

use crate::data;
use crate::data::Asset;

#[derive(Debug, Clone)]
pub struct State {
	pub echo: Echo,
	pub assets: Vec<Asset>,
}

impl State {
	pub fn latest(&self) -> Self {
		let mut next = self.clone();
		let mut chamber = self.echo.chamber().unwrap();
		next.assets = data::read_assets(&mut chamber).unwrap();
		next
	}
}


