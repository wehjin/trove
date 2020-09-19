use echo_lib::Echo;
use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, Padding, SenderLink, Spark, yard};
use yui::palette::FillColor;
use yui::yard::{ButtonState, Pressable};

use crate::data;
use crate::data::Asset;
use crate::edit_lot::EditLot;
use crate::list_assets::Action::AddLot;
use crate::view_asset::ViewAsset;

pub use self::action::*;
pub use self::state::*;

mod action;
mod state;

#[derive(Debug)]
pub struct ListAssets { echo: Echo }

impl ListAssets {
	pub fn new(echo: &Echo) -> Self { ListAssets { echo: echo.clone() } }
}

impl Spark for ListAssets {
	type State = State;
	type Action = Action;
	type Report = ();

	fn create(&self, _create: &Create<Self::Action, Self::Report>) -> Self::State {
		let echo = self.echo.to_owned();
		let mut chamber = echo.chamber().unwrap();
		State { echo, assets: data::read_assets(&mut chamber).unwrap() }
	}

	fn flow(&self, action: Self::Action, flow: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::Refresh => AfterFlow::Revise(flow.state().latest()),
			Action::ViewAsset(index) => {
				let link = flow.link().clone();
				flow.start_prequel(
					ViewAsset { asset: flow.state().assets[index].clone() },
					link.map(|_| Action::Refresh),
				);
				AfterFlow::Ignore
			}
			Action::AddLot(lot) => {
				data::add_lot(&lot, &flow.state().echo).unwrap();
				AfterFlow::Revise(flow.state().latest())
			}
			Action::CollectLot => {
				let link = flow.link().clone();
				flow.start_prequel(
					EditLot {},
					link.map(|lot| AddLot(lot)),
				);
				AfterFlow::Ignore
			}
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let column_width = 40;
		let asset_list = yard::list(LOT_LIST, 0, asset_list_items(&state.assets, link));
		let yard = asset_list.confine_width(column_width, Cling::Center).pad(1);
		Some(yard)
	}
}


impl Asset {
	fn as_item(&self, index: usize, link: &SenderLink<Action>) -> (u8, ArcYard) {
		let link = link.clone();
		let quad_text = QuadText {
			title: format!("{}", self.symbol),
			subtitle: format!("{}", self.corral),
			value: format!("{} {}", self.shares(), self.symbol),
			subvalue: format!("{} {}", self.lots.len(), if self.lots.len() == 1 { "lot" } else { "lots" }),
		};
		let yard = quad_label(&quad_text)
			.pad(1)
			.pressable(link.map(move |_| Action::ViewAsset(index)));
		(4u8, yard)
	}
}

const LOT_LIST: i32 = 50000;

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

fn asset_list_items(assets: &Vec<Asset>, link: &SenderLink<Action>) -> Vec<(u8, ArcYard)> {
	let mut items = assets.iter()
		.enumerate()
		.map(|(index, asset)| asset.as_item(index, link))
		.collect::<Vec<_>>();
	let add_lot_button = yard::button("Add Lot", ButtonState::enabled(link.map(|_| Action::CollectLot)));
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
