use chad_core::chad::Chad;
use chad_core::core::Squad;
use yui::{AfterFlow, ArcYard, Cling, Create, Flow, Padding, SenderLink, yard};
use yui::palette::StrokeColor;
use yui::yard::{MuxButton, Pressable};

use crate::{edit_lot, edit_member, edit_unspent, OWNER, render, YardId};
use crate::edit_squad::EditSquadSpark;

#[derive(Clone, Debug)]
pub struct State {
	pub squads: Vec<Squad>,
	pub pick: Option<(u64, Option<String>)>,
}

#[derive(Debug)]
pub enum Action {
	AddSquad,
	SquadAdded(u64),
	PickSquad(u64),
	AddMember(u64),
	MemberAdded((u64, String)),
	PickMember(u64, String),
	EditLot((u64, String, Option<u64>)),
	SetUnspent((u64, Option<f64>)),
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
				let snap = self.chad.snap();
				let squads = snap.squads(OWNER);
				let squad_exists = squads.iter().any(|it| it.id == id);
				match squad_exists {
					true => AfterFlow::Revise(State { squads, pick: Some((id, None)) }),
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
				let mut state = ctx.state().clone();
				state.squads = snap.squads(OWNER);
				state.pick = Some((squad_id, None));
				AfterFlow::Revise(state)
			}
			Action::PickMember(squad_id, member_symbol) => {
				let snap = self.chad.snap();
				let mut state = ctx.state().clone();
				state.squads = snap.squads(OWNER);
				state.pick = Some((squad_id, Some(member_symbol)));
				AfterFlow::Revise(state)
			}
			Action::EditLot((squad_id, member_symbol, lot_id)) => {
				let spark = edit_lot::Spark { chad: self.chad.clone(), squad_id, member_symbol, lot_id };
				ctx.start_prequel(spark, ctx.link().map(|(squad_id, symbol, _)| Action::PickMember(squad_id, symbol)));
				AfterFlow::Ignore
			}
			Action::SetUnspent((squad_id, unspent)) => {
				let spark = edit_unspent::Spark {
					chad: self.chad.clone(),
					squad_id,
					unspent,
				};
				ctx.start_prequel(spark, ctx.link().map(|(squad_id, _)| Action::PickSquad(squad_id)));
				AfterFlow::Ignore
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
		let center = match squad {
			None => yard::label("Add a squad", StrokeColor::CommentOnBackground, Cling::Center),
			Some(squad) => match member {
				None => render::squad(
					squad,
					link.map({
						let squad_id = squad.id;
						move |_| Action::AddMember(squad_id)
					}),
					link.map(move |(squad_id, symbol)| Action::PickMember(squad_id, symbol)),
					link.map(Action::SetUnspent),
				),
				Some(member) => {
					let index = squad.members.iter().position(|it| &it.symbol == member).expect("Member index");
					let member = &squad.members[index];
					render::member_view(member, &squad, link.map(Action::EditLot))
				}
			},
		};
		let sources = squads.iter().map(|it| {
			let squad_id = it.id;
			let squad_name = format!("{}", it.name);
			let yard =
				yard::label(squad_name, StrokeColor::BodyOnBackground, Cling::Center)
					.pad_cols(1)
					.pressable(link.map(move |_| Action::PickSquad(squad_id)))
				;
			(3, yard)
		}).collect();
		let yard = yard::mux(
			YardId::PickSquadList.as_i32(),
			center,
			sources,
			selected,
			MuxButton("Add Squad".into(), link.map(|_| Action::AddSquad)),
		);
		Some(yard)
	}
}
