use std::collections::HashMap;

use chad_core::core::{Amount, AssetCode};
use chad_core::portfolio::holding::Holding;
use chad_core::portfolio::lot::Lot;
use chad_core::portfolio::Portfolio;
use chad_core::storage_link::StorageLink;
use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, Padding, SenderLink, Spark, yard};
use yui::palette::FillColor;
use yui::yard::{ButtonState, Pressable};

use crate::edit_lot::EditLot;
use crate::YardId;

pub enum Action {
	AddLot,
	WriteLot(Lot),
	ViewAsset(usize),
}

#[derive(Debug, Clone)]
pub struct State { holdings: Vec<Holding>, prices: HashMap<AssetCode, Amount> }

#[derive(Debug)]
pub struct ListAssets { link: StorageLink }

impl ListAssets {
	pub fn new(link: &StorageLink) -> Self { ListAssets { link: link.clone() } }
}

impl Spark for ListAssets {
	type State = State;
	type Action = Action;
	type Report = ();

	fn create(&self, _create: &Create<Self::Action, Self::Report>) -> Self::State {
		let portfolio = self.link.latest_portfolio();
		State { holdings: portfolio.holdings(), prices: portfolio.prices() }
	}

	fn flow(&self, action: Self::Action, flow: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::AddLot => {
				let spark = EditLot { lot_id: rand::random(), start_values: Vec::new() };
				flow.start_prequel(spark, flow.link().map(Action::WriteLot));
				let latest = self.link.latest_portfolio();
				AfterFlow::Revise(State { holdings: latest.holdings(), prices: latest.prices() })
			}
			Action::WriteLot(lot) => {
				self.link.update_lot(lot.lot_id, &lot.asset_code, lot.share_count, &lot.custodian);
				let latest = self.link.latest_portfolio();
				AfterFlow::Revise(State { holdings: latest.holdings(), prices: latest.prices() })
			}
			Action::ViewAsset(index) => AfterFlow::Ignore,
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let column_width = 40;
		let yard = yard::list(YardId::AssetList.as_i32(), 0, list_items(&state.holdings, &state.prices, link))
			.confine_width(column_width, Cling::Center)
			.pad(1);
		Some(yard)
	}
}

fn list_items(holdings: &Vec<Holding>, prices: &HashMap<AssetCode, Amount>, link: &SenderLink<Action>) -> Vec<(u8, ArcYard)> {
	let mut items = holdings.iter()
		.enumerate()
		.map(|(index, holding)| {
			let quad_text = QuadText {
				title: match &holding.asset_code() {
					AssetCode::Common(s) => s.to_owned(),
					AssetCode::Custom(s) => s.to_owned(),
				},
				subtitle: format!("{} lots", holding.lots.len()),
				value: {
					let share_count: Amount = holding.lots.iter().map(|it| it.share_count).sum();
					format!("{:.2} units", share_count)
				},
				subvalue: format!("{:.0} USD", holding.holding_value(prices)),
			};
			let yard = quad_label(&quad_text)
				.pad(1)
				.pressable(link.map(move |_| Action::ViewAsset(index)));
			(4u8, yard)
		})
		.collect::<Vec<_>>();
	let add_lot_button = yard::button("Add Lot", ButtonState::enabled(link.map(|_| Action::AddLot)));
	items.push((3, add_lot_button));
	items
}

#[derive(Debug, Clone)]
pub struct QuadText {
	title: String,
	subtitle: String,
	value: String,
	subvalue: String,
}

impl QuadText {
	pub fn title(&self) -> &String { &self.title }
	pub fn subtitle(&self) -> &String { &self.subtitle }
	pub fn value(&self) -> &String { &self.value }
	pub fn subvalue(&self) -> &String { &self.subvalue }
}

fn quad_label(quad_text: &QuadText) -> ArcYard {
	yard::quad_label(
		quad_text.title(),
		quad_text.subtitle(),
		quad_text.value(),
		quad_text.subvalue(),
		15,
		FillColor::Background,
	)
}
