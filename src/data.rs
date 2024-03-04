use rand::random;
use crate::tools::views::scroll_list::ScrollListRowDisplay;

#[derive(Debug)]
pub enum AssetKind {
	Commodity,
	Stock,
	Etf,
}

impl AssetKind {
	pub fn as_str(&self) -> &'static str {
		match self {
			AssetKind::Commodity => "Commodity",
			AssetKind::Stock => "Stock",
			AssetKind::Etf => "Etf"
		}
	}
}

pub struct Asset {
	name: String,
	kind: AssetKind,
	symbol: String,
}

impl Asset {
	pub fn new(num: usize) -> Self {
		Self {
			name: "New Asset".to_string(),
			kind: AssetKind::Commodity,
			symbol: format!("CM-{}-{}", num, random::<u16>()),
		}
	}
	pub fn to_row_display(&self) -> ScrollListRowDisplay {
		ScrollListRowDisplay {
			col1: self.name.to_string(),
			col2: self.kind.as_str().to_string(),
			col3: self.symbol.to_string(),
		}
	}
}
