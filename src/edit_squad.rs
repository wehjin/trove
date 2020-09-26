use chad_core::chad::Chad;
use stringedit::{StringEdit, Validity};
use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, Pack, Padding, SenderLink, Spark, StringEditAction, yard};
use yui::palette::StrokeColor;
use yui::yard::ButtonState;

use crate::YardId;

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
			Action::Submit => {
				if ctx.state().is_valid() {
					let id = rand::random();
					self.chad.add_squad(id, &ctx.state().chars.iter().cloned().collect::<String>().trim(), self.owner);
					AfterFlow::Close(Some(id))
				} else {
					AfterFlow::Ignore
				}
			}
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let title = yard::title("Add Squad", StrokeColor::BodyOnBackground, Cling::LeftTop);
		let trellis = yard::list(
			YardId::EditSquadList.as_i32(),
			0,
			vec![(3, yard::textfield(YardId::NameField.as_i32(), "Name", state.clone(), link.map(Action::NameAction))), ],
		);
		let close = yard::button("Close", ButtonState::enabled(link.map(|_| Action::Close)));
		let submit = {
			if state.is_valid() {
				yard::button("Submit", ButtonState::enabled(link.map(|_| Action::Submit)))
			} else {
				yard::button("Submit", ButtonState::disabled())
			}
		};
		let yard = trellis
			.pack_top(4, title)
			.pack_right(12, close.confine(11, 1, Cling::RightTop))
			.pack_bottom(2, submit.confine(16, 1, Cling::Bottom));
		Some(yard.pad(2))
	}
}