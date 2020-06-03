use echo_lib::Echo;
use yui::{AfterFlow, ArcYard, Cling, Confine, Flow, Link, Pack, Padding, Spark, yard};
use yui::palette::{FillColor, StrokeColor};
use yui::yard::Pressable;

use crate::{data, QuadText};
use crate::data::{Asset, Lot};
use crate::edit_lot::EditLot;
use crate::list_assets::Action::AddLot;
use crate::view_asset::ViewAsset;

pub struct ListAssets { echo: Echo }

impl ListAssets {
	pub fn new(echo: &Echo) -> Self { ListAssets { echo: echo.clone() } }
}

#[derive(Debug, Clone)]
pub struct State {
	echo: Echo,
	pub assets: Vec<Asset>,
}

impl State {
	fn latest(&self) -> Self {
		let mut next = self.clone();
		next.assets = data::assets(self.echo.chamber().unwrap().objects::<Lot>().unwrap());
		next
	}
}

impl Spark for ListAssets {
	type State = State;
	type Action = Action;
	type Report = ();

	fn create(&self, _report_link: Option<Link<Self::Report>>) -> Self::State {
		let echo = self.echo.to_owned();
		let lots = echo.chamber().unwrap().objects::<Lot>().unwrap();
		State { echo, assets: data::assets(lots) }
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

	fn yard(state: &Self::State, link: &Link<Self::Action>) -> Option<ArcYard> {
		let column_width = 40;
		let list_items = {
			let mut items = state.assets.iter()
				.enumerate()
				.map(|(index, asset)| asset.as_item(index, link))
				.collect::<Vec<_>>();
			let add_lot_button = {
				let link = link.to_owned();
				yard::button_enabled("Add Lot", move |_| link.send(Action::CollectLot))
			};
			items.push((3, add_lot_button));
			items
		};
		let list = yard::list(LOT_LIST, 0, list_items);
		let title = yard::title("Assets", StrokeColor::BodyOnBackground, Cling::Left).pad(1);
		let content = list
			.pack_top(4, title)
			.confine_width(column_width, Cling::Left);
		Some(content.pad(1))
	}
}

pub enum Action {
	Refresh,
	ViewAsset(usize),
	AddLot(Lot),
	CollectLot,
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
