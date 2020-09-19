use crate::data::Asset;

#[derive(Debug)]
pub struct Faction {
	pub id: String,
	pub assets: Vec<Asset>,
}

impl Faction {
	pub fn name(&self) -> &str { &self.id }
}
