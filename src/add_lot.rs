use chad_core::chad::Chad;
use yui::{AfterFlow, ArcYard, Create, Flow, SenderLink, yard};
use yui::yard::ButtonState;

use crate::render;

#[derive(Clone, Debug)]
pub struct State {}

pub enum Action { Close, Submit }

pub struct Spark {
	pub chad: Chad,
	pub squad_id: u64,
	pub member_symbol: String,
}

impl yui::Spark for Spark {
	type State = State;
	type Action = Action;
	type Report = (u64, String, u64);

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		State {}
	}

	fn flow(&self, action: Self::Action, _ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::Close => AfterFlow::Close(None),
			Action::Submit => AfterFlow::Close(None),
		}
	}

	fn render(_state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let yard = render::dialog("Add Lot", link.map(|_| Action::Close), ButtonState::disabled(), yard::empty());
		Some(yard)
	}
}

