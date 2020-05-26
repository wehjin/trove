use echo_lib::Echo;
use yui::{AfterFlow, ArcYard, Cling, Confine, Flow, Link, Pack, Padding, Spark, yard};
use yui::palette::{FillColor, StrokeColor};
use yui::yard::Pressable;

use crate::data::Lot;
use crate::edit_lot::EditLot;
use crate::list_lots::Action::AddLot;
use crate::QuadText;

pub struct ListLots { echo: Echo }

impl ListLots {
	pub fn new(echo: &Echo) -> Self { ListLots { echo: echo.clone() } }
}

#[derive(Debug, Clone)]
pub struct State {
	echo: Echo,
	pub lots: Vec<Lot>,
}

impl State {
	fn latest(&self) -> Self {
		let lots = self.echo.chamber().unwrap().objects::<Lot>().unwrap();
		let mut next = self.clone();
		next.lots = lots;
		next
	}
}

pub enum Action {
	CollectLot,
	AddLot(Lot),
}

impl Spark for ListLots {
	type State = State;
	type Action = Action;
	type Report = ();

	fn create(&self, _report_link: Option<Link<Self::Report>>) -> Self::State {
		let echo = self.echo.to_owned();
		let lots = echo.chamber().unwrap().objects::<Lot>().unwrap();
		State { echo, lots }
	}

	fn flow(flow: &impl Flow<Self::State, Self::Action, Self::Report>, action: Self::Action) -> AfterFlow<Self::State> {
		match action {
			Action::CollectLot => {
				let link = flow.link().clone();
				flow.start_prequel(EditLot {}, move |lot| link.send(AddLot(lot)));
				AfterFlow::Ignore
			}
			Action::AddLot(lot) => {
				let echo = &flow.state().echo;
				echo.write(|write| write.writable(&lot)).unwrap();
				let state = flow.state().latest();
				AfterFlow::Revise(state)
			}
		}
	}

	fn yard(state: &Self::State, link: &Link<Self::Action>) -> Option<ArcYard> {
		let column_width = 40;
		let button = {
			let link = link.to_owned();
			yard::button_enabled("Add Lot", move |_| link.send(Action::CollectLot))
		};
		let items = {
			let mut items = state.lots.iter().map(|lot| {
				lot.as_item(link)
			}).collect::<Vec<_>>();
			items.push((3, button));
			items
		};
		let list = yard::list(LOT_LIST, 0, items);
		let title = yard::title("Lots", StrokeColor::BodyOnBackground, Cling::Left).pad(1);
		let content = list
			.pack_top(4, title)
			.confine_width(column_width, Cling::Left);
		Some(content.pad(1))
	}
}

impl Lot {
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
			.pressable(move |_| link.send(Action::CollectLot));
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
