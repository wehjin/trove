use chad_core::chad::Chad;
use stringedit::{StringEdit, Validity};
use yui::{AfterFlow, ArcYard, Create, Flow, Link, SenderLink, Spark, StringEditAction,  Trigger, yard};
use yui::yard::{ButtonAction, ButtonModel, Priority};
use yui::yard::model::{ScrollAction, ScrollModel};

use crate::{render, YardId};

pub struct EditSquadSpark {
	pub(crate) chad: Chad,
	pub(crate) owner: u64,
}

#[derive(Debug, Clone)]
pub enum DialogPart { Close, Submit, Delete }

#[derive(Debug, Clone)]
pub struct DialogModel {
	pub close: ButtonModel,
	pub submit: ButtonModel,
	pub delete: Option<ButtonModel>,
	action_link: SenderLink<DialogAction>,
	submit_trigger: Trigger,
	close_trigger: Trigger,
	delete_trigger: Option<Trigger>,
}

#[derive(Debug, Clone)]
pub enum DialogAction {
	Press(DialogPart),
	Release(DialogPart),
}

impl DialogModel {
	pub fn new(close_trigger: Trigger, submit_trigger: Trigger, action_link: SenderLink<DialogAction>) -> Self {
		DialogModel {
			close: ButtonModel::enabled(
				"x",
				action_link.to_trigger(DialogAction::Release(DialogPart::Close)),
				action_link.to_sync().map(|_| DialogAction::Press(DialogPart::Close)),
				Priority::Default,
			),
			submit: ButtonModel::disabled("Submit", action_link.to_trigger(DialogAction::Release(DialogPart::Submit))),
			delete: None,
			action_link,
			submit_trigger,
			close_trigger,
			delete_trigger: None,
		}
	}
	pub fn update(self, action: DialogAction) -> Self {
		let (action, part) = match action {
			DialogAction::Press(part) => (ButtonAction::Press, part),
			DialogAction::Release(part) => (ButtonAction::Release, part),
		};
		if let ButtonAction::Release = action {
			match part {
				DialogPart::Close => self.close_trigger.send(()),
				DialogPart::Submit => self.submit_trigger.send(()),
				DialogPart::Delete => if let Some(delete_trigger) = &self.delete_trigger {
					delete_trigger.send(())
				},
			}
		}
		match part {
			DialogPart::Close => DialogModel { close: self.close.update(action), ..self },
			DialogPart::Submit => DialogModel { submit: self.submit.update(action), ..self },
			DialogPart::Delete => DialogModel { delete: self.delete.map(|delete| delete.update(action)), ..self },
		}
	}

	pub fn enable_submit(self, enable: bool) -> Self {
		let submit = if enable {
			let press_link = self.action_link.to_sync().map(|_| DialogAction::Press(DialogPart::Submit));
			self.submit.enable("Submit", press_link)
		} else {
			let submit = self.submit;
			ButtonModel::disabled(&submit.label, submit.release_trigger)
		};
		DialogModel { submit, ..self }
	}

	pub fn enable_delete(self, label: &str, trigger: Trigger) -> Self {
		let delete = ButtonModel::enabled(
			label,
			self.action_link.to_trigger(DialogAction::Release(DialogPart::Delete)),
			self.action_link.to_sync().map(|_| DialogAction::Press(DialogPart::Delete)),
			Priority::None,
		);
		let delete_trigger = Some(trigger);
		DialogModel { delete: Some(delete), delete_trigger, ..self }
	}
}

#[derive(Clone)]
pub enum Action {
	Close,
	UpdateEdit(StringEditAction),
	Submit,
	UpdateScroll(ScrollAction),
	UpdateDialog(DialogAction),
}

impl Spark for EditSquadSpark {
	type State = (StringEdit, ScrollModel, DialogModel);
	type Action = Action;
	type Report = u64;

	fn create(&self, ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let edit = StringEdit::new("", 0, Validity::NotEmpty);
		let scroll = ScrollModel::new(YardId::EditSquadList.as_i32(), vec![3], 0);
		let dialog = DialogModel::new(
			ctx.link().to_trigger(Action::Close),
			ctx.link().to_trigger(Action::Submit),
			ctx.link().map(|action| Action::UpdateDialog(action)),
		);
		(edit, scroll, dialog)
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		let (edit, scroll, dialog) = ctx.state().clone();
		match action {
			Action::Close => AfterFlow::Close(None),
			Action::UpdateEdit(action) => {
				let edit = edit.edit(action);
				let dialog = dialog.enable_submit(edit.is_valid());
				AfterFlow::Revise((edit, scroll, dialog))
			}
			Action::Submit => if edit.is_valid() {
				let id = rand::random();
				self.chad.add_squad(id, &edit.chars.iter().cloned().collect::<String>().trim(), self.owner);
				AfterFlow::Close(Some(id))
			} else {
				AfterFlow::Ignore
			},
			Action::UpdateScroll(action) => {
				let scroll = if let Some(scroll) = scroll.update(action) {
					scroll
				} else {
					scroll
				};
				AfterFlow::Revise((edit, scroll, dialog))
			}
			Action::UpdateDialog(action) => {
				let dialog = dialog.update(action);
				AfterFlow::Revise((edit, scroll, dialog))
			}
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let (edit, scroll, dialog) = &state.clone();
		let editor = yard::textfield(scroll.id, "Name", edit.clone(), link.map(Action::UpdateEdit));
		let content = yard::list(vec![editor], scroll.clone(), link.to_sync().map(|action| Action::UpdateScroll(action)));
		let render = render::dialog("Add Squad", dialog, content);
		Some(render)
	}
}