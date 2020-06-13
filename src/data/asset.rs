use std::collections::HashMap;

use crate::data::Lot;

#[cfg(test)]
mod tests {
	use std::error::Error;

	use crate::data::{Asset, Lot};
	use crate::data;

	#[test]
	fn form_assets() -> Result<(), Box<dyn Error>> {
		let aapl_main = Lot::new("aapl", "main", "schwab", "fang", 1);
		let amzn = Lot::new("amzn", "main", "schwab", "fang", 2);
		let aapl_alt = Lot::new("aapl", "alt", "schwab", "fang", 1);

		let lots = vec![aapl_main.to_owned(), amzn.to_owned(), aapl_alt.to_owned()];
		let assets = data::assets_from_lots(lots);
		assert_eq!(assets.len(), 2);
		assert_eq!(
			assets[0],
			Asset { symbol: "aapl".to_string(), corral: "fang".to_string(), lots: vec![aapl_main.to_owned(), aapl_alt.to_owned()] }
		);
		assert_eq!(
			assets[1],
			Asset { symbol: "amzn".to_string(), corral: "fang".to_string(), lots: vec![amzn.to_owned()] }
		);
		Ok(())
	}
}

pub fn assets_from_lots(lots: Vec<Lot>) -> Vec<Asset> {
	let mut lot_groups = HashMap::new();
	for lot in lots {
		let key = (lot.symbol().to_string(), lot.corral().to_string());
		let mut asset_lots = match lot_groups.remove(&key) {
			None => Vec::new(),
			Some(lots) => lots,
		};
		asset_lots.push(lot);
		lot_groups.insert(key, asset_lots);
	}
	let mut assets: Vec<Asset> = lot_groups.into_iter().map(|((symbol, corral), lots)| Asset { symbol, corral, lots }).collect();
	assets.sort_by(|a, b| {
		a.symbol.partial_cmp(&b.symbol).unwrap()
			.then_with(|| a.corral.partial_cmp(&b.corral).unwrap())
			.then_with(|| a.lots.len().partial_cmp(&b.lots.len()).unwrap())
	});
	assets
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Asset {
	pub symbol: String,
	pub corral: String,
	pub lots: Vec<Lot>,
}

impl Asset {
	pub fn shares(&self) -> u64 { self.lots.iter().fold(0, |sum, next| sum + next.shares()) }
}
