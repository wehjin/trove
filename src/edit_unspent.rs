use chad_core::chad::Chad;
use stringedit::Validity;
use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, SenderLink, StringEdit, StringEditAction, yard};
use yui::yard::ButtonState;

use crate::render;
use crate::YardId::UnspentEdit;

#[derive(Debug, Clone)]
pub struct State {
	pub string_edit: StringEdit
}

#[derive(Debug)]
pub enum Action {
	Close,
	Edit(StringEditAction),
	Submit,
}

#[derive(Debug)]
pub struct Spark {
	pub chad: Chad,
	pub squad_id: u64,
	pub unspent: Option<f64>,
}

impl yui::Spark for Spark {
	type State = State;
	type Action = Action;
	type Report = (u64, f64);

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let string = match self.unspent {
			None => String::new(),
			Some(unspent) => format!("{}", unspent),
		};
		State { string_edit: StringEdit::new(string, 0, Validity::Double) }
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::Close => {
				AfterFlow::Close(None)
			}
			Action::Edit(action) => {
				AfterFlow::Revise(State { string_edit: ctx.state().string_edit.edit(action) })
			}
			Action::Submit => {
				let string = ctx.state().string_edit.chars.iter().cloned().collect::<String>();
				let unspent = string.parse::<f64>().expect("parse f64");
				self.chad.set_unspent(self.squad_id, unspent);
				AfterFlow::Close(Some((self.squad_id, unspent)))
			}
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let yard = render::dialog(
			"Set Unspent",
			link.map(|_| Action::Close),
			if state.string_edit.is_valid() { ButtonState::enabled(link.map(|_| Action::Submit)) } else { ButtonState::disabled() },
			None,
			yard::textfield(UnspentEdit.as_i32(), "Unspent", state.string_edit.clone(), link.map(Action::Edit)).confine_height(3, Cling::Top),
		);
		Some(yard)
	}
}