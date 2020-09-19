use std::collections::HashMap;
use std::io;

use echo_lib::{Chamber, Echo};

use crate::data::asset::assets_from_lots;

pub use self::asset::Asset;
pub use self::faction::Faction;
pub use self::lot::*;

pub mod path;
mod asset;
mod lot;
mod faction;

#[cfg(test)]
mod tests {
	use std::error::Error;

	use super::*;

	#[test]
	fn happy() -> Result<(), Box<dyn Error>> {
		let echo = echo("happy")?;
		add_lot(&Lot::new("tsla", "ira", "square", "focus", 10), &echo)?;
		add_lot(&Lot::new("tsla", "main", "square", "focus", 5), &echo)?;
		add_lot(&Lot::new("tsla", "main", "morgan", "focus", 12), &echo)?;
		add_lot(&Lot::new("spce", "main", "morgan", "spread", 1), &echo)?;
		add_lot(&Lot::new("niu", "main", "morgan", "spread", 1), &echo)?;
		{
			let assets = read_assets(&mut echo.chamber()?)?;
			let tsla = find_asset("tsla", &assets).unwrap();
			assert_eq!(tsla.shares(), 27);
		}
		{
			let factions = read_factions(&mut echo.chamber()?)?;
			assert_eq!(factions.len(), 3, "FACTIONS: {:?}", factions);
			let faction = find_faction("spread", &factions).unwrap();
			assert_eq!(faction.assets.len(), 2);
			assert_eq!(faction.name(), "spread");
		}
		Ok(())
	}

	fn find_faction<'a>(faction_id: &str, factions: &'a Vec<Faction>) -> Option<&'a Faction> {
		factions.iter().find(|it| faction_id.eq(&it.id))
	}

	fn find_asset<'a>(symbol: &str, assets: &'a Vec<Asset>) -> Option<&'a Asset> {
		assets.iter().find(|it| symbol.eq(&it.symbol))
	}
}

pub fn read_factions(chamber: &mut Chamber) -> io::Result<Vec<Faction>> {
	let mut map: HashMap<String, Vec<Asset>> = HashMap::new();
	let assets = read_assets(chamber)?;
	for asset in assets.into_iter() {
		let faction_id = asset.corral.to_owned();
		let mut faction_assets = match map.get(&faction_id) {
			None => Vec::new(),
			Some(existing) => existing.to_vec()
		};
		faction_assets.push(asset);
		map.insert(faction_id, faction_assets);
	}
	let factions: Vec<Faction> = map.into_iter().map(|(id, assets)| Faction { id, assets }).collect();
	Ok(factions)
}

pub fn read_assets(chamber: &mut Chamber) -> io::Result<Vec<Asset>> {
	let lots = chamber.objects::<Lot>()?;
	Ok(assets_from_lots(lots))
}

pub fn add_lot(lot: &Lot, echo: &Echo) -> io::Result<Chamber> {
	echo.write(|write| write.writable(lot)).unwrap();
	echo.chamber()
}

pub fn echo(folder_name: &str) -> io::Result<Echo> {
	let folder_path = path::echo(folder_name)?;
	let echo = Echo::connect("v1", &folder_path);
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
