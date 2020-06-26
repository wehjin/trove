use echo_lib::Echo;
use yui::{AfterFlow, ArcYard, Before, Create, Flow, Link, Pack, Spark, Story, yard};
use yui::palette::FillColor;
use yui::yard::Tab;

use crate::list_assets::ListAssets;
use crate::list_factions::ListFactions;
use crate::YardId;

#[derive(Debug)]
pub struct MainPage { echo: Echo }

impl MainPage {
	pub fn new(echo: Echo) -> Self { MainPage { echo } }
}

impl Spark for MainPage {
	type State = State;
	type Action = Action;
	type Report = ();

	fn render(state: &Self::State, link: &Link<Self::Action>) -> Option<ArcYard> {
		let tabbar = yard::tabbar(MAIN_TABS, tab_index(&state.active_tab), link.callback(|select| Action::SelectTab(MAIN_TABS[select])));
		let banner = tabbar.before(yard::fill(FillColor::Primary));
		let body = match state.active_tab {
			MainTab::Assets => yard::publisher(&state.list_assets),
			MainTab::Factions => yard::publisher(&state.list_factions),
		};
		let yard = body.pack_top(3, banner);
		Some(yard)
	}


	fn flow(&self, flow: &impl Flow<Self::State, Self::Action, Self::Report>, action: Self::Action) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::SelectTab(tab) => AfterFlow::Revise(State { active_tab: tab, ..flow.state().clone() }),
		}
	}

	fn create(&self, create: &Create<Self::Action, Self::Report>) -> Self::State {
		State {
			active_tab: MainTab::Assets,
			list_assets: ListAssets::new(&self.echo).spark(create.edge().clone(), None),
			list_factions: ListFactions::new(&self.echo).spark(create.edge().clone(), None),
		}
	}
}

#[derive(Debug, Clone)]
pub struct State {
	active_tab: MainTab,
	list_assets: Story<ListAssets>,
	list_factions: Story<ListFactions>,
}

const MAIN_TABS: &[MainTab] = &[MainTab::Assets, MainTab::Factions];

fn tab_index(tab: &MainTab) -> usize {
	match tab {
		MainTab::Assets => 0,
		MainTab::Factions => 1,
	}
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
	SelectTab(MainTab)
}

#[derive(Debug, Copy, Clone)]
pub enum MainTab { Assets, Factions }

impl Tab for MainTab {
	fn uid(&self) -> i32 {
		let yard_id = match self {
			MainTab::Assets => YardId::AssetsTab,
			MainTab::Factions => YardId::FactionsTab,
		};
		yard_id.as_i32()
	}

	fn label(&self) -> &str {
		match self {
			MainTab::Assets => "Assets",
			MainTab::Factions => "Factions",
		}
	}
}

