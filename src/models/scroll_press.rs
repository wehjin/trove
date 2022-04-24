use rand::random;
use yui::{Link, SenderLink, SyncLink};
use yui::yard::{PressAction, PressModel};
use yui::yard::model::{ScrollAction, ScrollModel};

#[derive(Debug, Clone)]
pub enum ScrollPressAction {
	Press(usize),
	Release(usize),
	UpdateScroll(ScrollAction),
}

#[derive(Debug, Clone)]
pub struct ScrollPressModel {
	pub scroll_model: ScrollModel,
	pub press_models: Vec<PressModel>,
	action_link: SenderLink<ScrollPressAction>,
	on_release_link: SenderLink<usize>,
	height: u8,
}

impl ScrollPressModel {
	pub fn new(id: i32, action_link: SenderLink<ScrollPressAction>, count: usize, height: u8, selected_index: usize, on_release_link: SenderLink<usize>) -> Self {
		ScrollPressModel {
			scroll_model: ScrollModel::new_count_height(id, 0, 1, 0),
			press_models: Vec::new(),
			action_link,
			on_release_link,
			height,
		}.with_count_selected_index(count, selected_index)
	}
	pub fn update(self, action: ScrollPressAction) -> Self {
		match action {
			ScrollPressAction::Press(index) => {
				let mut press_models = self.press_models;
				press_models[index] = press_models[index].update(PressAction::Press);
				ScrollPressModel { press_models, ..self }
			}
			ScrollPressAction::Release(index) => {
				let mut press_models = self.press_models;
				press_models[index] = press_models[index].update(PressAction::Release);
				self.on_release_link.send(index);
				ScrollPressModel { press_models, ..self }
			}
			ScrollPressAction::UpdateScroll(action) => {
				let scroll_model = if let Some(updated_scroll_model) = self.scroll_model.update(action) {
					updated_scroll_model
				} else {
					self.scroll_model
				};
				ScrollPressModel { scroll_model, ..self }
			}
		}
	}
	pub fn with_count_selected_index(self, count: usize, selected_index: usize) -> Self {
		let scroll_model = ScrollModel::new_count_height(self.scroll_model.id, count, self.height, selected_index);
		let mut press_models = self.press_models;
		for index in 0..count {
			if index >= press_models.len() {
				let press_model = PressModel::new(random(), self.action_link.to_trigger(ScrollPressAction::Release(index)));
				press_models.push(press_model)
			}
		}
		ScrollPressModel { scroll_model, press_models, ..self }
	}
	pub fn press_link_at_index(&self, index: usize) -> SyncLink<i32> {
		self.action_link.to_sync().map(move |_| ScrollPressAction::Press(index))
	}
	pub fn scroll_link(&self) -> SyncLink<ScrollAction> {
		self.action_link.to_sync().map(|action| ScrollPressAction::UpdateScroll(action))
	}
}
