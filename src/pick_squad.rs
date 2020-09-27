use chad_core::chad::Chad;
use chad_core::core::Squad;
use yui::{AfterFlow, ArcYard, Before, Cling, Confine, Create, Flow, Pack, Padding, SenderLink, yard};
use yui::palette::{FillColor, StrokeColor};
use yui::yard::{ButtonState, Pressable};

use crate::{edit_member, OWNER, render, sprint, YardId};
use crate::edit_squad::EditSquadSpark;

#[derive(Clone, Debug)]
pub struct State {
	pub squads: Vec<Squad>,
	pub pick: Option<u64>,
	pub message: String,
}

#[derive(Debug)]
pub enum Action {
	AddMember(u64),
	MemberAdded((u64, String)),
	AddSquad,
	SquadAdded(u64),
	PickSquad(u64),
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
		let pick = squads.first().map(|it| it.id);
		State { squads, pick, message: "".to_string() }
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
				state.pick = Some(squad_id);
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
					pick: Some(id),
					message: "".to_string(),
				};
				AfterFlow::Revise(state)
			}
			Action::PickSquad(id) => {
				let mut state = ctx.state().clone();
				let squad_exists = state.squads.iter().any(|it| it.id == id);
				if squad_exists {
					state.message = "".to_string();
					state.pick = Some(id);
					AfterFlow::Revise(state)
				} else {
					state.message = format!("Invalid id: {}", id);
					AfterFlow::Revise(state)
				}
			}
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let mut squads = state.squads.clone();
		squads.sort_by_key(|it| it.name.to_owned());
		let selected = match state.pick {
			Some(id) => squads.iter().position(|it| it.id == id).unwrap_or(0),
			None => 0,
		};
		let content = {
			if selected >= squads.len() {
				yard::label("Add a squad", StrokeColor::CommentOnBackground, Cling::Center)
			} else {
				let squad = &squads[selected];
				let title = yard::title(&squad.name, StrokeColor::BodyOnPrimary, Cling::LeftBottom);
				let header = title.pad(1).before(yard::fill(FillColor::Primary));
				let content = {
					let unspent = {
						let label_text = "Unspent: ";
						let label = yard::label(label_text, StrokeColor::CommentOnBackground, Cling::Left);
						let button_text = sprint::amount(squad.unspent);
						let button = yard::button(&button_text, ButtonState::default(SenderLink::ignore()));
						yard::empty()
							.pack_left(button_text.len() as i32 + 6, button)
							.pack_left(label_text.len() as i32, label)
					};
					let members = {
						let member_count = squad.members.len();
						let label_text = format!("Members ({})", member_count);
						let label = yard::label(label_text, StrokeColor::CommentOnBackground, Cling::LeftBottom);
						let member_list = if member_count == 0 {
							yard::label("No members", StrokeColor::CommentOnBackground, Cling::Center)
						} else {
							let items = squad.members.iter()
								.enumerate()
								.rev()
								.map(|(index, it)| {
									(4, render::member_summary(it, index))
								}).collect();
							yard::list(YardId::SquadMembersList.as_i32(), 0, items)
						};
						let button_text = "Add Member";
						let button = yard::button(button_text, ButtonState::enabled(link.map({
							let squad_id = squad.id;
							move |_| Action::AddMember(squad_id)
						})));
						member_list
							.pack_top(1, label)
							.pack_bottom(3, button)
					};
					members.pack_top(3, unspent)
				}.pad(1);
				content.pack_top(4, header)
			}
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
		if state.message.is_empty() {
			Some(yard)
		} else {
			Some(yard.pack_bottom(2, yard::label(&state.message, StrokeColor::BodyOnBackground, Cling::LeftBottom)))
		}
	}
}
