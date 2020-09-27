use std::io::empty;

use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, Pack, Padding, SenderLink, yard};
use yui::palette::{FillColor, StrokeColor};
use yui::yard::ButtonState;

#[derive(Clone, Debug)]
pub(crate) struct State;

pub(crate) enum Action { Close }

pub(crate) struct Spark {
	pub squad_id: u64,
}

impl yui::Spark for Spark {
	type State = State;
	type Action = Action;
	type Report = (u64, u64);

	fn create(&self, ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		State {}
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::Close => AfterFlow::Close(None),
		}
	}

	fn render(_state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let title = yard::title("Add Member", StrokeColor::BodyOnBackground, Cling::LeftBottom).pack_top(1, yard::empty());
		let close = yard::button("Close", ButtonState::enabled(link.map(|_| Action::Close)));
		let header = title.pad_cols(2).pack_right(11, close);
		let content = yard::empty();
		let render = content.pack_top(3, header);
		Some(render)
	}
}


