use std::collections::HashMap;

use chad_core::chad::Chad;
use stringedit::{StringEdit, Validity};
use yui::{AfterFlow, ArcYard, Cling, Create, Flow, SenderLink, StringEditAction, yard};

use crate::{OWNER, render, YardId};
use crate::edit_squad::{DialogAction, DialogModel};

#[derive(Clone, Debug)]
pub(crate) struct State {
	symbol_edit: StringEdit,
	price_edit: StringEdit,
	prices: HashMap<String, f64>,
	dialog_model: DialogModel,
}

impl State {
	fn symbol(&self) -> String {
		self.symbol_edit.chars.iter().cloned().collect::<String>().to_uppercase()
	}
	fn price(&self) -> f64 {
		self.price_edit.chars.iter().clone().collect::<String>().parse::<f64>().unwrap_or(0.0)
	}
	fn can_submit(&self) -> bool {
		self.symbol_edit.is_valid() && self.price_edit.is_valid()
	}
}

#[derive(Clone)]
pub(crate) enum EditMemberAction {
	Close,
	EditSymbol(StringEditAction),
	EditPrice(StringEditAction),
	Submit,
	UpdateDialog(DialogAction),
}

pub(crate) struct Spark {
	pub chad: Chad,
	pub squad_id: u64,
}

impl yui::Spark for Spark {
	type State = State;
	type Action = EditMemberAction;
	type Report = (u64, String);

	fn create(&self, ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let prices = {
			let snap = self.chad.snap();
			let squads = snap.squads(OWNER);
			let squad_pos = squads.iter().position(|it| it.id == self.squad_id).expect("Squad exists");
			let squad = &squads[squad_pos];
			squad.prices.to_owned()
		};
		State {
			symbol_edit: StringEdit::empty(Validity::NotEmpty),
			price_edit: StringEdit::empty(Validity::Double),
			prices,
			dialog_model: DialogModel::new(
				ctx.link().to_trigger(EditMemberAction::Close),
				ctx.link().to_trigger(EditMemberAction::Submit),
				ctx.link().map(|action| EditMemberAction::UpdateDialog(action)),
			),
		}
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			EditMemberAction::Close => AfterFlow::Close(None),
			EditMemberAction::Submit => {
				let state = ctx.state();
				if state.price_edit.is_valid() && state.symbol_edit.is_valid() {
					let symbol = state.symbol();
					let price = state.price();
					self.chad.add_member(self.squad_id, &symbol, price);
					AfterFlow::Close(Some((self.squad_id, symbol)))
				} else {
					AfterFlow::Ignore
				}
			}
			EditMemberAction::EditSymbol(action) => {
				let mut state = ctx.state().clone();
				state.symbol_edit = state.symbol_edit.edit(action);
				let symbol = state.symbol();
				if state.price_edit.chars.is_empty() && state.prices.contains_key(&symbol) {
					let price = format!("{}", state.prices[&symbol]);
					let cursor_pos = price.chars().count();
					state.price_edit = StringEdit::new(price, cursor_pos, Validity::Double);
				}
				let can_submit = state.can_submit();
				state.dialog_model = state.dialog_model.enable_submit(can_submit);
				AfterFlow::Revise(state)
			}
			EditMemberAction::EditPrice(action) => {
				let mut state = ctx.state().clone();
				state.price_edit = state.price_edit.edit(action);
				let can_submit = state.can_submit();
				state.dialog_model = state.dialog_model.enable_submit(can_submit);
				AfterFlow::Revise(state)
			}
			EditMemberAction::UpdateDialog(action) => {
				let mut state = ctx.state().clone();
				state.dialog_model = state.dialog_model.update(action);
				AfterFlow::Revise(state)
			}
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let content = yard::trellis(3, 1, Cling::Top, vec![
			yard::textfield(
				YardId::MemberSymbolEdit.as_i32(),
				"Symbol",
				state.symbol_edit.clone(),
				link.map(EditMemberAction::EditSymbol),
			),
			yard::textfield(
				YardId::MemberPriceEdit.as_i32(),
				"Price",
				state.price_edit.clone(),
				link.map(EditMemberAction::EditPrice),
			),
		]);
		let render = render::dialog("Add Member", &state.dialog_model, content);
		Some(render)
	}
}


