use echo_lib::Echo;
use yui::{AfterFlow, ArcYard, Before, Create, Flow, Link, Pack, Spark, Story, yard};
use yui::palette::FillColor;
use yui::yard::Tab;

use crate::list_assets::ListAssets;
use crate::YardId;

#[derive(Debug)]
pub struct MainPage { echo: Echo }

impl MainPage {
	pub fn new(echo: Echo) -> Self { MainPage { echo } }
}

impl Spark for MainPage {
	type State = State;
	type Action = ();
	type Report = ();

	fn render(state: &Self::State, _link: &Link<Self::Action>) -> Option<ArcYard> {
		let body = yard::publisher(&state.list_assets);
		let yard = body.pack_top(3, banner());
		Some(yard)
	}


	fn flow(_flow: &impl Flow<Self::State, Self::Action, Self::Report>, _action: Self::Action) -> AfterFlow<Self::State, Self::Report> {
		AfterFlow::Ignore
	}

	fn create(&self, create: &Create<Self::Action, Self::Report>) -> Self::State {
		State {
			main_tab: MainTab::Assets,
			list_assets: ListAssets::new(&self.echo).spark(create.edge().clone(), None),
		}
	}
}

fn banner() -> ArcYard {
	let tabbar = yard::tabbar(MAIN_TABS, 0, |_select| {});
	tabbar.before(yard::fill(FillColor::Primary))
}

#[derive(Debug, Clone)]
pub struct State {
	main_tab: MainTab,
	list_assets: Story<ListAssets>,
}

#[derive(Debug, Clone)]
enum MainTab { Assets }

impl Tab for MainTab {
	fn uid(&self) -> i32 {
		match self { MainTab::Assets => YardId::AssetsTab.as_i32() }
	}

	fn label(&self) -> &str {
		match self { MainTab::Assets => "Assets" }
	}
}

const MAIN_TABS: &[MainTab] = &[MainTab::Assets];

