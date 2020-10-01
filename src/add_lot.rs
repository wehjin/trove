use chad_core::chad::Chad;
use stringedit::Validity;
use yui::{AfterFlow, ArcYard, Cling, Create, Flow, SenderLink, StringEdit, StringEditAction, yard};
use yui::yard::ButtonState;

use crate::render;
use crate::YardId::{LotAccountEdit, LotSharesEdit};

#[derive(Clone, Debug)]
pub struct State {
	symbol: String,
	account_edit: StringEdit,
	shares_edit: StringEdit,
}

pub enum Action { Close, Submit, EditAccount(StringEditAction), EditShares(StringEditAction) }

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
		State {
			symbol: self.member_symbol.to_owned(),
			account_edit: StringEdit::new("", 0, Validity::NotEmpty),
			shares_edit: StringEdit::new("", 0, Validity::Double),
		}
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::Close => AfterFlow::Close(None),
			Action::Submit => AfterFlow::Close(None),
			Action::EditAccount(action) => AfterFlow::Revise(State { account_edit: ctx.state().account_edit.edit(action), ..ctx.state().clone() }),
			Action::EditShares(action) => AfterFlow::Revise(State { shares_edit: ctx.state().shares_edit.edit(action), ..ctx.state().clone() }),
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let title = format!("Add {} Lot", &state.symbol);
		let content = yard::trellis(3, 1, Cling::Top, vec![
			yard::textfield(LotAccountEdit.as_i32(), "Account", state.account_edit.clone(), link.map(Action::EditAccount)),
			yard::textfield(LotSharesEdit.as_i32(), "Shares", state.shares_edit.clone(), link.map(Action::EditShares)),
		]);
		let submit_state = {
			if state.account_edit.is_valid() && state.shares_edit.is_valid() {
				ButtonState::enabled(link.map(|_| Action::Submit))
			} else {
				ButtonState::disabled()
			}
		};
		let yard = render::dialog(&title, link.map(|_| Action::Close), submit_state, content);
		Some(yard)
	}
}

