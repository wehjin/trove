use chad_core::{AssetCode, Lot};
use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, Padding, SenderLink, Spark, yard};
use yui::palette::FillColor;
use yui::yard::{ButtonState, Pressable};

use crate::{ChadLink, YardId};

pub enum Action {
	AddLot,
	Refresh,
	ViewAsset(usize),
	UpdateLot(Lot),
}

#[derive(Debug, Clone)]
pub struct State { lots: Vec<Lot> }

#[derive(Debug)]
pub struct ListAssets { link: ChadLink }

impl ListAssets {
	pub fn new(link: &ChadLink) -> Self { ListAssets { link: link.clone() } }
}

impl Spark for ListAssets {
	type State = State;
	type Action = Action;
	type Report = ();

	fn create(&self, _create: &Create<Self::Action, Self::Report>) -> Self::State {
		let portfolio = self.link.latest_portfolio();
		State { lots: portfolio.lots() }
	}

	fn flow(&self, action: Self::Action, flow: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::AddLot => {
				self.link.update_lot(
					rand::random(),
					&AssetCode::Common("SQ".to_string()),
					1.0,
					&"robinhood".to_string(),
					27.0,
				);
				flow.redraw(); // TODO: Fix this in yui.
				AfterFlow::Revise(State { lots: self.link.latest_portfolio().lots() })
			}
			Action::Refresh => AfterFlow::Revise(State { lots: self.link.latest_portfolio().lots() }),
			Action::ViewAsset(index) => AfterFlow::Ignore,
			Action::UpdateLot(lot) => AfterFlow::Ignore,
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let column_width = 40;
		let yard = yard::list(YardId::AssetList.as_i32(), 0, list_items(&state.lots, link))
			.confine_width(column_width, Cling::Center)
			.pad(1);
		Some(yard)
	}
}

fn list_items(lots: &Vec<Lot>, link: &SenderLink<Action>) -> Vec<(u8, ArcYard)> {
	let mut lot_items = lots.iter()
		.enumerate()
		.map(|(index, lot)| {
			let quad_text = QuadText {
				title: match &lot.asset_code {
					AssetCode::Common(s) => s.to_owned(),
					AssetCode::Custom(s) => s.to_owned(),
				},
				subtitle: format!("{}", lot.custodian),
				value: format!("{:.2} units", lot.share_count),
				subvalue: format!("{:.0} USD", lot.currency_value()),
			};
			let yard = quad_label(&quad_text)
				.pad(1)
				.pressable(link.map(move |_| Action::ViewAsset(index)));
			(4u8, yard)
		})
		.collect::<Vec<_>>();
	let add_lot_button = yard::button("Add Lot", ButtonState::enabled(link.map(|_| Action::AddLot)));
	lot_items.push((3, add_lot_button));
	lot_items
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
