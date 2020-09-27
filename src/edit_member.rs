use std::collections::HashMap;

use chad_core::chad::Chad;
use stringedit::{StringEdit, Validity};
use yui::{AfterFlow, ArcYard, Cling, Create, Flow, SenderLink, StringEditAction, yard};
use yui::yard::ButtonState;

use crate::{OWNER, render, YardId};

#[derive(Clone, Debug)]
pub(crate) struct State {
	symbol_edit: StringEdit,
	price_edit: StringEdit,
	prices: HashMap<String, f64>,
}

impl State {
	fn symbol(&self) -> String {
		self.symbol_edit.chars.iter().cloned().collect::<String>().to_uppercase()
	}
	fn price(&self) -> f64 {
		self.price_edit.chars.iter().clone().collect::<String>().parse::<f64>().unwrap_or(0.0)
	}
}

pub(crate) enum Action {
	Close,
	EditSymbol(StringEditAction),
	EditPrice(StringEditAction),
	Submit,
}

pub(crate) struct Spark {
	pub chad: Chad,
	pub squad_id: u64,
}

impl yui::Spark for Spark {
	type State = State;
	type Action = Action;
	type Report = (u64, String);

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
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
		}
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::Submit => {
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
			Action::Close => AfterFlow::Close(None),
			Action::EditSymbol(action) => {
				let mut state = ctx.state().clone();
				state.symbol_edit = state.symbol_edit.edit(action);
				let symbol = state.symbol();
				if state.price_edit.chars.is_empty() && state.prices.contains_key(&symbol) {
					let price = format!("{}", state.prices[&symbol]);
					let cursor_pos = price.chars().count();
					state.price_edit = StringEdit::new(price, cursor_pos, Validity::Double);
				}
				AfterFlow::Revise(state)
			}
			Action::EditPrice(action) => {
				let mut state = ctx.state().clone();
				state.price_edit = state.price_edit.edit(action);
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
				link.map(Action::EditSymbol),
			),
			yard::textfield(
				YardId::MemberPriceEdit.as_i32(),
				"Price",
				state.price_edit.clone(),
				link.map(Action::EditPrice),
			),
		]);
		let render = render::dialog(
			"Add Member",
			link.map(|_| Action::Close),
			if state.symbol_edit.is_valid() && state.price_edit.is_valid() {
				ButtonState::enabled(link.map(|_| Action::Submit))
			} else {
				ButtonState::disabled()
			},
			content,
		);
		Some(render)
	}
}


