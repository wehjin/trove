use std::collections::HashMap;

use chad_core::core::Squad;
use yui::{Link, SenderLink, SyncLink};
use yui::yard::{ PressModel};
use yui::yard::model::{ScrollAction, ScrollModel};

use crate::models::scroll_press::{ScrollPressAction, ScrollPressModel};

#[derive(Debug, Clone)]
pub enum SquadChoicesAction {
	ScrollPressReleased(usize),
	UpdateScrollPress(ScrollPressAction),
}

#[derive(Debug, Clone)]
pub struct SquadChoicesModel {
	scroll_press: ScrollPressModel,
	squad_selected: SenderLink<u64>,
	index_by_squad_id: HashMap<u64, usize>,
	squad_id_by_index: Vec<u64>,
}

impl SquadChoicesModel {
	pub fn new(id: i32, action_link: SenderLink<SquadChoicesAction>, squad_selected: SenderLink<u64>) -> Self {
		SquadChoicesModel {
			scroll_press: ScrollPressModel::new(
				id,
				action_link.map(|action| SquadChoicesAction::UpdateScrollPress(action)),
				0, 3, 0,
				action_link.map(|index| SquadChoicesAction::ScrollPressReleased(index)),
			),
			squad_selected,
			index_by_squad_id: HashMap::new(),
			squad_id_by_index: Vec::new(),
		}
	}
	pub fn press_model_and_link_for_squad(&self, squad_id: u64) -> (&PressModel, SyncLink<i32>) {
		let index = *self.index_by_squad_id.get(&squad_id).expect("index for squad id");
		let model = &self.scroll_press.press_models[index];
		let link = self.scroll_press.press_link_at_index(index);
		(model, link)
	}
	pub fn scroll_model_and_link(&self) -> (&ScrollModel, SyncLink<ScrollAction>) {
		let model = &self.scroll_press.scroll_model;
		let link = self.scroll_press.scroll_link();
		(model, link)
	}
	pub fn with_pick(self, squads: &Vec<Squad>, pick: &Option<(u64, Option<String>)>) -> Self {
		let index_by_squad_id
			= squads.iter()
			.enumerate()
			.map(|(index, squad)| (squad.id, index))
			.collect::<HashMap<_, _>>();
		let squad_id_by_index
			= squads.iter()
			.map(|squad| squad.id)
			.collect::<Vec<_>>();
		let count = squad_id_by_index.len();
		let selection_index = match pick {
			None => 0,
			Some((squad_id, _)) => index_by_squad_id[squad_id],
		};
		let scroll_press = self.scroll_press.with_count_selected_index(count, selection_index);
		SquadChoicesModel { scroll_press, index_by_squad_id, squad_id_by_index, ..self }
	}
	pub fn update(self, action: SquadChoicesAction) -> Self {
		match action {
			SquadChoicesAction::ScrollPressReleased(index) => {
				let squad_id = self.squad_id_by_index[index];
				self.squad_selected.send(squad_id);
				self
			}
			SquadChoicesAction::UpdateScrollPress(action) => {
				let scroll_press = self.scroll_press.update(action);
				SquadChoicesModel { scroll_press, ..self }
			}
		}
	}
}


// pub fn squad_choices_scroll_for_pick(&self, pick: &Option<(u64, Option<String>)>, squads: &Vec<Squad>) -> ScrollModel {
// 	let scroll_id = self.squad_choices_scroll_press.scroll_model.id;
// 	if let Some((squad_id, _)) = pick {
// 		if let Some(index) = squads.iter().position(|squad| squad.id == *squad_id) {
// 			ScrollModel::new_count_height(scroll_id, squads.len(), 3, index)
// 		} else {
// 			ScrollModel::new_count_height(scroll_id, squads.len(), 3, 0)
// 		}
// 	} else {
// 		ScrollModel::new_count_height(scroll_id, squads.len(), 3, 0)
// 	}
// }
