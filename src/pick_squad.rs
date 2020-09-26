use chad_core::chad::Chad;
use chad_core::core::Squad;
use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, Pack, Padding, SenderLink, yard};
use yui::palette::StrokeColor;
use yui::yard::{ButtonState, Pressable};

use crate::{OWNER, YardId};
use crate::edit_squad::EditSquadSpark;

#[derive(Clone, Debug)]
pub struct Spark { pub chad: Chad }

pub enum Action { AddSquad, SquadAdded(u64), PickSquad(u64) }

#[derive(Clone, Debug)]
pub struct State { pub squads: Vec<Squad>, pub pick: Option<u64> }

impl yui::Spark for Spark {
	type State = State;
	type Action = Action;
	type Report = ();

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let snap = self.chad.snap();
		let squads = snap.squads(OWNER);
		let pick = squads.first().map(|it| it.id);
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
					pick: Some(id),
				};
				AfterFlow::Revise(state)
			}
			Action::PickSquad(_id) => AfterFlow::Ignore,
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let content = yard::label("Empty", StrokeColor::CommentOnBackground, Cling::Center);
		let side = {
			let button = yard::button("Add Squad", ButtonState::enabled(link.map(|_| Action::AddSquad)));
			let content = if state.squads.is_empty() {
				button.confine_height(3, Cling::Center)
			} else {
				let mut squads = state.squads.clone();
				squads.sort_by_key(|it| it.name.to_owned());
				let items: Vec<(u8, ArcYard)> = squads.iter().map(|it| {
					let yard = yard::label(&it.name, StrokeColor::BodyOnBackground, Cling::Center)
						.pad_cols(1)
						.pressable(link.map({
							let id = it.id;
							move |_| Action::PickSquad(id)
						}));
					(3, yard)
				}).collect();
				let selected = match state.pick {
					Some(id) => squads.iter().position(|it| it.id == id).unwrap_or(0),
					None => 0,
				};
				let list = yard::list(YardId::PickSquadList.as_i32(), selected, items);
				list.pack_bottom(3, button)
			};
			content
		};
		let yard = content.pack_left(30, side);
		Some(yard)
	}
}
