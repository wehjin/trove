use echo_lib::Echo;

use crate::data;
use crate::data::{Asset, Lot};

#[derive(Debug, Clone)]
pub struct State {
	pub echo: Echo,
	pub assets: Vec<Asset>,
}

impl State {
	pub fn latest(&self) -> Self {
		let mut next = self.clone();
		next.assets = data::assets(self.echo.chamber().unwrap().objects::<Lot>().unwrap());
		next
	}
}


