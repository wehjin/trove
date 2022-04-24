

use chad_core::chad::{Chad, Snap};
use chad_core::core::Squad;

use yui::{AfterFlow, ArcYard, Cling, Create, Flow, Link, Padding, SenderLink,  yard};
use yui::palette::StrokeColor;
use yui::yard::{ButtonAction, ButtonModel,  Priority};


use crate::{edit_lot, edit_member, edit_unspent, OWNER,  YardId};
use crate::edit_squad::EditSquadSpark;

use crate::models::squad_choices::{SquadChoicesAction, SquadChoicesModel};


#[derive(Debug, Clone)]
pub struct State {
	pub squads: Vec<Squad>,
	pub pick: Option<(u64, Option<String>)>,
	pub squad_choices: SquadChoicesModel,
	pub add_squad_press: ButtonModel,
}

impl State {
	pub fn squads_for_snap(snap: &Snap) -> Vec<Squad> {
		let mut squads = snap.squads(OWNER);
		squads.sort_by_key(|squad| squad.name.to_owned());
		squads
	}
}

#[derive(Debug, Clone)]
pub enum Action {
	AddSquad,
	SquadAdded(u64),
	PickSquad(u64),
	AddMember(u64),
	MemberAdded((u64, String)),
	PickMember(u64, String),
	AddLot,
	EditLot((u64, String, Option<u64>)),
	SetUnspent((u64, Option<f64>)),
	UpdateAddSquadPress(ButtonAction),
	UpdateSquadChoices(SquadChoicesAction),
}

#[derive(Clone, Debug)]
pub struct Spark {
	pub chad: Chad,
}

impl yui::Spark for Spark {
	type State = State;
	type Action = Action;
	type Report = ();

	fn create(&self, ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let snap = self.chad.snap();
		let squads = State::squads_for_snap(&snap);
		let pick
			= squads.first()
			.map(|squad| (squad.id, None));
		// let add_lot_trigger = ctx.link().to_trigger(Action::AddLot);
		// SquadMemberViewModel::new();

		let squad_choices = SquadChoicesModel::new(
			YardId::PickSquadList.as_i32(),
			ctx.link().map(|action| Action::UpdateSquadChoices(action)),
			ctx.link().map(|squad_id| Action::PickSquad(squad_id)),
		);
		let add_squad_press = ButtonModel::enabled(
			"Add Squad",
			ctx.link().to_trigger(Action::AddSquad),
			ctx.link().to_sync().map(|_| Action::UpdateAddSquadPress(ButtonAction::Press)),
			Priority::None,
		);

		State { squads, pick, squad_choices, add_squad_press }
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		let state = ctx.state().clone();
		match action {
			Action::AddSquad => {
				let spark = EditSquadSpark { chad: self.chad.clone(), owner: OWNER };
				ctx.start_prequel(spark, ctx.link().map(Action::SquadAdded));
				let add_squad_press = state.add_squad_press.update(ButtonAction::Release);
				AfterFlow::Revise(State { add_squad_press, ..state })
			}
			Action::SquadAdded(id) => {
				let snap = self.chad.snap();
				let squads = State::squads_for_snap(&snap);
				let pick = Some((id, None));
				let squad_choices = state.squad_choices.with_pick(&squads, &pick);
				AfterFlow::Revise(State { squads, pick, squad_choices, ..state })
			}
			Action::PickSquad(id) => {
				let snap = self.chad.snap();
				let squads = State::squads_for_snap(&snap);
				let squad_exists = squads.iter().any(|it| it.id == id);
				match squad_exists {
					true => {
						let pick = Some((id, None));
						// TODO: let member_lots_scroll = state.member_lots_scroll_for_pick(&pick, &squads);
						let squad_choices = state.squad_choices.with_pick(&squads, &pick);
						let state = State { squads, pick, squad_choices, ..state };
						AfterFlow::Revise(state)
					}
					false => AfterFlow::Ignore,
				}
			}
			Action::AddMember(squad_id) => {
				let spark = edit_member::Spark { chad: self.chad.clone(), squad_id };
				ctx.start_prequel(spark, ctx.link().map(Action::MemberAdded));
				AfterFlow::Ignore
			}
			Action::MemberAdded((squad_id, _member_id)) => {
				let snap = self.chad.snap();
				let squads = State::squads_for_snap(&snap);
				let pick = Some((squad_id, None));
				let squad_choices = state.squad_choices.with_pick(&state.squads, &state.pick);
				// TODO: state.member_lots_scroll = state.member_lots_scroll_for_pick(&state.pick, &state.squads);
				AfterFlow::Revise(State { squads, pick, squad_choices, ..state })
			}
			Action::PickMember(squad_id, member_symbol) => {
				let snap = self.chad.snap();
				let squads = State::squads_for_snap(&snap);
				let pick = Some((squad_id, Some(member_symbol)));
				let squad_choices = state.squad_choices.with_pick(&state.squads, &state.pick);
				// TODO: state.member_lots_scroll = state.member_lots_scroll_for_pick(&state.pick, &state.squads);
				AfterFlow::Revise(State { squads, pick, squad_choices, ..state })
			}
			Action::AddLot => {
				if let Some((squad_id, symbol)) = &state.pick {
					if let Some(index) = state.squads.iter().position(|squad| squad.id == *squad_id) {
						let squad = &state.squads[index];
						if let Some(symbol) = symbol {
							let member_index = squad.members.iter().position(|member| &member.symbol == symbol).expect("Member index");
							let member = &squad.members[member_index];
							ctx.link().send(Action::EditLot((member.squad_id, member.symbol.clone(), None)));
						}
					}
				}
				// TODO: let add_lot_press = state.add_lot_press.update(ButtonAction::Release);
				// TODO: AfterFlow::Revise(State { add_lot_press, ..state })
				AfterFlow::Ignore
			}
			Action::EditLot((squad_id, member_symbol, lot_id)) => {
				let spark = edit_lot::Spark { chad: self.chad.clone(), squad_id, member_symbol, lot_id };
				ctx.start_prequel(spark, ctx.link().map(|(squad_id, symbol, _)| Action::PickMember(squad_id, symbol)));
				AfterFlow::Ignore
			}
			Action::SetUnspent((squad_id, unspent)) => {
				let spark = edit_unspent::Spark { chad: self.chad.clone(), squad_id, unspent };
				ctx.start_prequel(spark, ctx.link().map(|(squad_id, _)| Action::PickSquad(squad_id)));
				AfterFlow::Ignore
			}
			Action::UpdateSquadChoices(action) => {
				let squad_choices = state.squad_choices.update(action);
				AfterFlow::Revise(State { squad_choices, ..state })
			}
			Action::UpdateAddSquadPress(action) => {
				let add_squad_press = state.add_squad_press.update(action);
				AfterFlow::Revise(State { add_squad_press, ..state })
			}
		}
	}

	fn render(state: &Self::State, _link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let squads = &state.squads;
		let (selected, member) = match &state.pick {
			Some((id, member)) => (squads.iter().position(|it| it.id == *id).unwrap_or(0), member),
			None => (0, &None),
		};
		let squad = if selected < squads.len() { Some(&squads[selected]) } else { None };
		let center = match squad {
			None => yard::label("Add a squad", StrokeColor::CommentOnBackground, Cling::Center),
			Some(_squad) => match member {
				None => {
					// TODO: Render squad
					// render::squad(
					// 	squad,
					// 	link.map(move |(squad_id, symbol)| Action::PickMember(squad_id, symbol)),
					// 	&state.unspent_press,
					// 	&state.members_scroll,
					// 	&state.add_member_press,
					// )
					yard::label("Squad View", StrokeColor::BodyOnBackground, Cling::Left)
				}
				Some(_member) => {
					// TODO: Render member
					// let index = squad.members.iter().position(|it| &it.symbol == member).expect("Member index");
					// let member = &squad.members[index];
					// let scroll_link = link.to_sync().map(|action| Action::UpdateMemberLotsScroll(action));
					// render::member_view(member, &squad, &state.member_lots_scroll, scroll_link, &state.add_lot_press)
					yard::label("Member View", StrokeColor::BodyOnBackground, Cling::Left)
				}
			},
		};
		let squad_item_yards = squads
			.iter()
			.enumerate()
			.map(|(_index, squad)| {
				let content = yard::label(format!("{}", squad.name), StrokeColor::BodyOnBackground, Cling::Center).pad_cols(1);
				let (press_model, press_link) = state.squad_choices.press_model_and_link_for_squad(squad.id);
				yard::pressable(content, press_model, press_link)
			})
			.collect::<Vec<_>>();

		let (choices_scroll_model, choices_scroll_link) = state.squad_choices.scroll_model_and_link();
		let yard = yard::mux(
			center,
			squad_item_yards,
			state.add_squad_press.clone(),
			choices_scroll_model.clone(),
			choices_scroll_link,
		);
		Some(yard)
	}
}
