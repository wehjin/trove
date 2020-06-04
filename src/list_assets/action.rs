use crate::data::Lot;

pub enum Action {
	Refresh,
	ViewAsset(usize),
	AddLot(Lot),
	CollectLot,
}