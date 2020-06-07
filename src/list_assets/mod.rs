use echo_lib::Echo;
use yui::{AfterFlow, ArcYard, Before, Cling, Confine, Create, Flow, Link, Pack, Padding, Spark, yard};
use yui::palette::FillColor;
use yui::yard::{Pressable, Tab};

use crate::{data, QuadText, YardId};
use crate::data::{Asset, Lot};
use crate::edit_lot::EditLot;
use crate::list_assets::Action::AddLot;
use crate::view_asset::ViewAsset;

pub use self::action::*;
pub use self::state::*;

mod action;
mod state;

pub struct ListAssets { echo: Echo }

impl ListAssets {
	pub fn new(echo: &Echo) -> Self { ListAssets { echo: echo.clone() } }
}

impl Spark for ListAssets {
	type State = State;
	type Action = Action;
	type Report = ();

	fn yard(state: &Self::State, link: &Link<Self::Action>) -> Option<ArcYard> {
		let column_width = 40;
		let asset_list = yard::list(LOT_LIST, 0, asset_list_items(&state.assets, link));
		let content = asset_list.confine_width(column_width, Cling::Center).pad(1)
			.pack_top(3, banner());
		Some(content)
	}

	fn flow(flow: &impl Flow<Self::State, Self::Action, Self::Report>, action: Self::Action) -> AfterFlow<Self::State> {
		match action {
			Action::Refresh => AfterFlow::Revise(flow.state().latest()),
			Action::ViewAsset(index) => {
				let link = flow.link().clone();
				flow.start_prequel(
					ViewAsset { asset: flow.state().assets[index].clone() },
					move |_| link.send(Action::Refresh),
				);
				AfterFlow::Ignore
			}
			Action::AddLot(lot) => {
				flow.state().echo.write(|write| write.writable(&lot)).unwrap();
				AfterFlow::Revise(flow.state().latest())
			}
			Action::CollectLot => {
				let link = flow.link().clone();
				flow.start_prequel(
					EditLot {},
					move |lot| link.send(AddLot(lot)),
				);
				AfterFlow::Ignore
			}
		}
	}

	fn create(&self, _create: &Create<Self::Action, Self::Report>) -> Self::State {
		let echo = self.echo.to_owned();
		let lots = echo.chamber().unwrap().objects::<Lot>().unwrap();
		State { echo, assets: data::assets(lots) }
	}
}


impl Asset {
	fn as_item(&self, index: usize, link: &Link<Action>) -> (u8, ArcYard) {
		let link = link.clone();
		let quad_text = QuadText {
			title: format!("{}", self.symbol),
			subtitle: format!("{}", self.corral),
			value: format!("{} {}", self.shares(), self.symbol),
			subvalue: format!("{} {}", self.lots.len(), if self.lots.len() == 1 { "lot" } else { "lots" }),
		};
		let yard = quad_label(&quad_text)
			.pad(1)
			.pressable(move |_| link.send(Action::ViewAsset(index)));
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

fn banner() -> ArcYard {
	let tabbar = yard::tabbar(MAIN_TABS, 0, |_select| {});
	tabbar.before(yard::fill(FillColor::Primary))
}


enum MainTab {
	Assets
}

impl Tab for MainTab {
	fn uid(&self) -> i32 {
		match self { MainTab::Assets => YardId::AssetsTab.as_i32() }
	}

	fn label(&self) -> &str {
		match self { MainTab::Assets => "Assets" }
	}
}

const MAIN_TABS: &[MainTab] = &[MainTab::Assets];

fn asset_list_items(assets: &Vec<Asset>, link: &Link<Action>) -> Vec<(u8, ArcYard)> {
	let mut items = assets.iter()
		.enumerate()
		.map(|(index, asset)| asset.as_item(index, link))
		.collect::<Vec<_>>();
	let add_lot_button = {
		let link = link.to_owned();
		yard::button_enabled("Add Lot", move |_| link.send(Action::CollectLot))
	};
	items.push((3, add_lot_button));
	items
}
