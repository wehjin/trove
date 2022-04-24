use chad_core::chad::Chad;
use stringedit::Validity;
use yui::{AfterFlow, ArcYard, Cling, Create, Flow, Pack, SenderLink, StringEdit, StringEditAction, yard};
use yui::palette::StrokeColor;

use crate::{OWNER, render};
use crate::edit_squad::{DialogAction, DialogModel};
use crate::YardId::{LotAccountEdit, LotSharesEdit};

#[derive(Clone, Debug)]
pub struct State {
	symbol: String,
	add_lot: bool,
	account_edit: StringEdit,
	shares_edit: StringEdit,
	dialog_model: DialogModel,
}

impl State {
	fn is_valid(&self) -> bool {
		self.account_edit.is_valid() && self.shares_edit.is_valid()
	}
}

#[derive(Clone)]
pub enum EditLotAction {
	Close,
	Submit,
	EditAccount(StringEditAction),
	EditShares(StringEditAction),
	Delete,
	UpdateDialog(DialogAction),
}

pub struct Spark {
	pub chad: Chad,
	pub squad_id: u64,
	pub member_symbol: String,
	pub lot_id: Option<u64>,
}

impl yui::Spark for Spark {
	type State = State;
	type Action = EditLotAction;
	type Report = (u64, String, Option<u64>);

	fn create(&self, ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let (init_account, init_shares) = match self.lot_id {
			None => ("".to_string(), "".to_string()),
			Some(lot_id) => {
				let squad = self.chad.snap().squads(OWNER).into_iter().find(|it| it.id == self.squad_id).expect("Squad exists");
				let lot = squad.lots.into_iter().find(|it| it.id == lot_id).expect("Lot exists");
				(lot.account.clone(), format!("{}", lot.shares))
			}
		};
		let add_lot = self.lot_id.is_none();
		let dialog_model = {
			let model = DialogModel::new(
				ctx.link().to_trigger(EditLotAction::Close),
				ctx.link().to_trigger(EditLotAction::Submit),
				ctx.link().map(|action| EditLotAction::UpdateDialog(action)),
			);
			if add_lot { model } else {
				model.enable_delete("Delete", ctx.link().to_trigger(EditLotAction::Delete))
			}
		};
		State {
			symbol: self.member_symbol.to_owned(),
			add_lot,
			account_edit: StringEdit::new(init_account.clone(), init_account.len(), Validity::NotEmpty),
			shares_edit: StringEdit::new(init_shares.clone(), init_shares.len(), Validity::Double),
			dialog_model,
		}
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		let mut state = ctx.state().clone();
		match action {
			EditLotAction::Close => AfterFlow::Close(None),
			EditLotAction::Submit => {
				if state.is_valid() {
					let account = state.account_edit.chars.iter().cloned().collect::<String>().trim().to_owned();
					let shares = state.shares_edit.chars.iter().cloned().collect::<String>().parse::<f64>().expect("Float in shares_edit");
					let lot_id = self.lot_id.unwrap_or_else(rand::random);
					self.chad.add_lot(self.squad_id, lot_id, &self.member_symbol, &account, shares);
					let lot_path = (self.squad_id, self.member_symbol.to_owned(), Some(lot_id));
					AfterFlow::Close(Some(lot_path))
				} else {
					AfterFlow::Close(None)
				}
			}
			EditLotAction::EditAccount(action) => {
				state.account_edit = state.account_edit.edit(action);
				let is_valid = state.is_valid();
				state.dialog_model = state.dialog_model.enable_submit(is_valid);
				AfterFlow::Revise(state)
			}
			EditLotAction::EditShares(action) => {
				state.shares_edit = state.shares_edit.edit(action);
				let is_valid = state.is_valid();
				state.dialog_model = state.dialog_model.enable_submit(is_valid);
				AfterFlow::Revise(state)
			}
			EditLotAction::Delete => match self.lot_id {
				None => AfterFlow::Ignore,
				Some(lot_id) => {
					self.chad.del_lot(self.squad_id, lot_id);
					AfterFlow::Close(Some((self.squad_id, self.member_symbol.to_owned(), None)))
				}
			},
			EditLotAction::UpdateDialog(action) => {
				let dialog_model = state.dialog_model.update(action);
				AfterFlow::Revise(State { dialog_model, ..state })
			}
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let title = if state.add_lot { format!("Add Lot") } else { format!("Edit Lot") };
		let content = yard::trellis(3, 1, Cling::Top, vec![
			yard::textfield(LotAccountEdit.as_i32(), "Account", state.account_edit.clone(), link.map(EditLotAction::EditAccount)),
			yard::textfield(LotSharesEdit.as_i32(), "Shares", state.shares_edit.clone(), link.map(EditLotAction::EditShares)),
		]).pack_top(2, yard::label(&state.symbol, StrokeColor::BodyOnBackground, Cling::LeftTop));
		let yard = render::dialog(&title, &state.dialog_model, content);
		Some(yard)
	}
}

