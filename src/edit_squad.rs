use chad_core::chad::Chad;
use stringedit::{StringEdit, Validity};
use yui::{AfterFlow, ArcYard, Create, Flow, SenderLink, Spark, StringEditAction, yard};
use yui::yard::ButtonState;

use crate::{render, YardId};

pub struct EditSquadSpark {
	pub(crate) chad: Chad,
	pub(crate) owner: u64,
}

pub enum Action {
	Close,
	NameAction(StringEditAction),
	Submit,
}

impl Spark for EditSquadSpark {
	type State = StringEdit;
	type Action = Action;
	type Report = u64;

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		StringEdit::new("", 0, Validity::NotEmpty)
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::Close => AfterFlow::Close(None),
			Action::NameAction(action) => {
				let name_edit = ctx.state().edit(action);
				AfterFlow::Revise(name_edit)
			}
			Action::Submit => if ctx.state().is_valid() {
				let id = rand::random();
				self.chad.add_squad(id, &ctx.state().chars.iter().cloned().collect::<String>().trim(), self.owner);
				AfterFlow::Close(Some(id))
			} else {
				AfterFlow::Ignore
			},
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let trellis = yard::list(
			YardId::EditSquadList.as_i32(),
			0,
			vec![(3, yard::textfield(YardId::NameField.as_i32(), "Name", state.clone(), link.map(Action::NameAction))), ],
		);
		let render = render::dialog(
			"Add Squad",
			link.map(|_| Action::Close),
			if state.is_valid() {
				ButtonState::enabled(link.map(|_| Action::Submit))
			} else {
				ButtonState::disabled()
			},
			None,
			trellis,
		);
		Some(render)
	}
}