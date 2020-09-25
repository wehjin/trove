use chad_core::chad::{Chad, Snap};
use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, Pack, Padding, SenderLink, Spark, yard};
use yui::palette::StrokeColor;
use yui::yard::{ButtonState, Pressable};

use crate::{OWNER, YardId};
use crate::edit_squad::EditSquadSpark;

#[derive(Clone, Debug)]
pub struct PickSquadSpark { pub chad: Chad }

pub enum Action {
	AddSquad,
	SquadAdded(u64),
	PickSquad(u64),
}

impl Spark for PickSquadSpark {
	type State = Snap;
	type Action = Action;
	type Report = ();

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let snap = self.chad.snap();
		snap
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::AddSquad => {
				let spark = EditSquadSpark { chad: self.chad.clone(), owner: OWNER };
				ctx.start_prequel(spark, ctx.link().map(Action::SquadAdded));
				AfterFlow::Ignore
			}
			Action::SquadAdded(_id) => AfterFlow::Revise(self.chad.snap()),
			Action::PickSquad(_id) => AfterFlow::Ignore,
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let content = yard::label("Empty", StrokeColor::CommentOnBackground, Cling::Center);
		let side = {
			let button = yard::button("Add Squad", ButtonState::enabled(link.map(|_| Action::AddSquad)));
			let squads = state.squads(OWNER);
			let content = if squads.is_empty() {
				button.confine_height(3, Cling::Center)
			} else {
				let mut items: Vec<(u8, ArcYard)> = squads.iter().map(|it| {
					let yard = yard::label(&it.name, StrokeColor::BodyOnBackground, Cling::Center)
						.pad_cols(1)
						.pressable(link.map({
							let id = it.id;
							move |_| Action::PickSquad(id)
						}));
					(3, yard)
				}).collect();
				items.push((3, button));
				yard::list(YardId::PickSquadList.as_i32(), 0, items)
			};
			content
		};
		let yard = content.pack_left(30, side);
		Some(yard)
	}
}
