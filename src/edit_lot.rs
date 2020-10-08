use chad_core::chad::Chad;
use stringedit::Validity;
use yui::{AfterFlow, ArcYard, Cling, Create, Flow, Pack, SenderLink, StringEdit, StringEditAction, yard};
use yui::palette::StrokeColor;
use yui::yard::ButtonState;

use crate::{OWNER, render};
use crate::YardId::{LotAccountEdit, LotSharesEdit};

#[derive(Clone, Debug)]
pub struct State {
	symbol: String,
	add_lot: bool,
	account_edit: StringEdit,
	shares_edit: StringEdit,
}

impl State {
	fn is_valid(&self) -> bool {
		self.account_edit.is_valid() && self.shares_edit.is_valid()
	}
}

pub enum Action { Close, Submit, EditAccount(StringEditAction), EditShares(StringEditAction) }

pub struct Spark {
	pub chad: Chad,
	pub squad_id: u64,
	pub member_symbol: String,
	pub lot_id: Option<u64>,
}

impl yui::Spark for Spark {
	type State = State;
	type Action = Action;
	type Report = (u64, String, u64);

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let (init_account, init_shares) = match self.lot_id {
			None => ("".to_string(), "".to_string()),
			Some(lot_id) => {
				let squad = self.chad.snap().squads(OWNER).into_iter().find(|it| it.id == self.squad_id).expect("Squad exists");
				let lot = squad.lots.into_iter().find(|it| it.id == lot_id).expect("Lot exists");
				(lot.account.clone(), format!("{}", lot.shares))
			}
		};
		State {
			symbol: self.member_symbol.to_owned(),
			add_lot: self.lot_id.is_none(),
			account_edit: StringEdit::new(init_account.clone(), init_account.len(), Validity::NotEmpty),
			shares_edit: StringEdit::new(init_shares.clone(), init_shares.len(), Validity::Double),
		}
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::Close => AfterFlow::Close(None),
			Action::Submit => {
				if ctx.state().is_valid() {
					let account = ctx.state().account_edit.chars.iter().cloned().collect::<String>().trim().to_owned();
					let shares = ctx.state().shares_edit.chars.iter().cloned().collect::<String>().parse::<f64>().expect("Float in shares_edit");
					let lot_id = self.lot_id.unwrap_or_else(rand::random);
					self.chad.add_lot(self.squad_id, lot_id, &self.member_symbol, &account, shares);
					let report = (self.squad_id, self.member_symbol.to_owned(), lot_id);
					AfterFlow::Close(Some(report))
				} else {
					AfterFlow::Close(None)
				}
			}
			Action::EditAccount(action) => AfterFlow::Revise(State { account_edit: ctx.state().account_edit.edit(action), ..ctx.state().clone() }),
			Action::EditShares(action) => AfterFlow::Revise(State { shares_edit: ctx.state().shares_edit.edit(action), ..ctx.state().clone() }),
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let title = if state.add_lot { format!("Add Lot") } else { format!("Edit Lot") };
		let content = yard::trellis(3, 1, Cling::Top, vec![
			yard::textfield(LotAccountEdit.as_i32(), "Account", state.account_edit.clone(), link.map(Action::EditAccount)),
			yard::textfield(LotSharesEdit.as_i32(), "Shares", state.shares_edit.clone(), link.map(Action::EditShares)),
		]).pack_top(2, yard::label(&state.symbol, StrokeColor::BodyOnBackground, Cling::LeftTop));
		let submit_state = {
			if state.is_valid() {
				ButtonState::enabled(link.map(|_| Action::Submit))
			} else {
				ButtonState::disabled()
			}
		};
		let yard = render::dialog(&title, link.map(|_| Action::Close), submit_state, content);
		Some(yard)
	}
}

