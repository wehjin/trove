use chad_core::chad::Chad;
use chad_core::core::Squad;
use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, Pack, Padding, SenderLink, yard};
use yui::palette::StrokeColor;
use yui::yard::{ButtonState, Pressable};

use crate::{edit_member, OWNER, render, YardId};
use crate::edit_squad::EditSquadSpark;

#[derive(Clone, Debug)]
pub struct State {
	pub squads: Vec<Squad>,
	pub pick: Option<(u64, Option<String>)>,
}

#[derive(Debug)]
pub enum Action {
	AddMember(u64),
	MemberAdded((u64, String)),
	AddSquad,
	SquadAdded(u64),
	PickSquad(u64),
	PickMember(u64, String),
}

#[derive(Clone, Debug)]
pub struct Spark { pub chad: Chad }

impl yui::Spark for Spark {
	type State = State;
	type Action = Action;
	type Report = ();

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let snap = self.chad.snap();
		let squads = snap.squads(OWNER);
		let pick = squads.first().map(|it| it.id).map(|it| (it, None));
		State { squads, pick }
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::AddMember(squad_id) => {
				let spark = edit_member::Spark { chad: self.chad.clone(), squad_id };
				ctx.start_prequel(spark, ctx.link().map(Action::MemberAdded));
				AfterFlow::Ignore
			}
			Action::MemberAdded((squad_id, _member_id)) => {
				let snap = self.chad.snap();
				let mut state = ctx.state().clone();
				state.squads = snap.squads(OWNER);
				state.pick = Some((squad_id, None));
				AfterFlow::Revise(state)
			}
			Action::AddSquad => {
				let spark = EditSquadSpark { chad: self.chad.clone(), owner: OWNER };
				ctx.start_prequel(spark, ctx.link().map(Action::SquadAdded));
				AfterFlow::Ignore
			}
			Action::SquadAdded(id) => {
				let snap = self.chad.snap();
				let state = State {
					squads: snap.squads(OWNER),
					pick: Some((id, None)),
				};
				AfterFlow::Revise(state)
			}
			Action::PickSquad(id) => {
				let mut state = ctx.state().clone();
				let squad_exists = state.squads.iter().any(|it| it.id == id);
				match squad_exists {
					true => {
						state.pick = Some((id, None));
						AfterFlow::Revise(state)
					}
					false => AfterFlow::Ignore,
				}
			}
			Action::PickMember(squad_id, symbol) => {
				let mut state = ctx.state().clone();
				state.pick = Some((squad_id, Some(symbol)));
				AfterFlow::Revise(state)
			}
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let mut squads = state.squads.clone();
		squads.sort_by_key(|it| it.name.to_owned());
		let (selected, member) = match &state.pick {
			Some((id, member)) => (squads.iter().position(|it| it.id == *id).unwrap_or(0), member),
			None => (0, &None),
		};
		let squad = if selected < squads.len() { Some(&squads[selected]) } else { None };
		let content = match squad {
			None => yard::label("Add a squad", StrokeColor::CommentOnBackground, Cling::Center),
			Some(squad) => match member {
				None => render::squad(
					squad,
					link.map({
						let squad_id = squad.id;
						move |_| Action::AddMember(squad_id)
					}),
					link.map({
						move |(squad_id, symbol)| Action::PickMember(squad_id, symbol)
					}),
				),
				Some(member) => yard::label(member, StrokeColor::BodyOnBackground, Cling::Center),
			},
		};
		let side = {
			let button = yard::button("Add Squad", ButtonState::enabled(link.map(|_| Action::AddSquad)));
			if state.squads.is_empty() {
				button.confine_height(3, Cling::Center)
			} else {
				let items: Vec<(u8, ArcYard)> = squads.iter()
					.map(|it| {
						let squad_id = it.id;
						let squad_name = format!("{}", it.name);
						let yard = yard::label(squad_name, StrokeColor::BodyOnBackground, Cling::Center)
							.pad_cols(1)
							.pressable(link.map(move |_| Action::PickSquad(squad_id)));
						(3, yard)
					}).collect();
				let list = yard::list(YardId::PickSquadList.as_i32(), selected, items);
				list.pack_bottom(3, button)
			}
		};
		let yard = content.pack_left(30, side);
		Some(yard)
	}
}
