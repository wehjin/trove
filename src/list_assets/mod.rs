use echo_lib::Echo;
use yui::{AfterFlow, ArcYard, Cling, Confine, Flow, Link, Pack, Padding, Spark, yard};
use yui::palette::{FillColor, StrokeColor};
use yui::yard::Pressable;

use crate::data::Asset;
use crate::edit_asset::EditAsset;
use crate::list_assets::Action::AddAsset;
use crate::QuadText;

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
		let assets = self.echo.chamber().unwrap().objects::<Asset>().unwrap();
		let mut next = self.clone();
		next.assets = assets;
		next
	}
}

pub enum Action {
	CollectAsset,
	AddAsset(Asset),
}

impl Spark for ListAssets {
	type State = State;
	type Action = Action;
	type Report = ();

	fn create(&self, _report_link: Option<Link<Self::Report>>) -> Self::State {
		let echo = self.echo.to_owned();
		let assets = echo.chamber().unwrap().objects::<Asset>().unwrap();
		State { echo, assets }
	}

	fn flow(flow: &impl Flow<Self::State, Self::Action, Self::Report>, action: Self::Action) -> AfterFlow<Self::State> {
		match action {
			Action::CollectAsset => {
				let link = flow.link().clone();
				flow.start_prequel(EditAsset {}, move |asset| link.send(AddAsset(asset)));
				AfterFlow::Ignore
			}
			Action::AddAsset(asset) => {
				let echo = &flow.state().echo;
				echo.write(|write| write.writable(&asset)).unwrap();
				let state = flow.state().latest();
				AfterFlow::Revise(state)
			}
		}
	}

	fn yard(state: &Self::State, link: &Link<Self::Action>) -> Option<ArcYard> {
		let column_width = 40;
		let button = {
			let link = link.to_owned();
			yard::button_enabled("Add Asset", move |_| link.send(Action::CollectAsset))
		};
		let items = {
			let mut items = state.assets.iter().map(|asset| {
				asset.as_item(link)
			}).collect::<Vec<_>>();
			items.push((3, button));
			items
		};
		let list = yard::list(ASSET_LIST, 0, items);
		let title = yard::title("Assets", StrokeColor::BodyOnBackground, Cling::Left).pad(1);
		let content = list
			.pack_top(4, title)
			.confine_width(column_width, Cling::Left);
		Some(content.pad(1))
	}
}

impl Asset {
	fn as_item(&self, link: &Link<Action>) -> (u8, ArcYard) {
		let link = link.clone();
		let quad_text = QuadText {
			title: format!("{} - {}", self.symbol(), self.corral()),
			subtitle: format!("{}", self.corral()),
			value: format!("{} {}", self.shares(), self.symbol()),
			subvalue: format!("{}/{}", self.custodian(), self.account()),
		};
		let yard = quad_label(&quad_text)
			.pad(1)
			.pressable(move |_| link.send(Action::CollectAsset));
		(4u8, yard)
	}
}

const ASSET_LIST: i32 = 50000;

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
