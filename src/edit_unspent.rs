use chad_core::chad::Chad;
use stringedit::Validity;
use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, SenderLink, StringEdit, StringEditAction, yard};

use crate::edit_squad::{DialogAction, DialogModel};
use crate::render;
use crate::YardId::UnspentEdit;

#[derive(Debug, Clone)]
pub struct State {
	pub string_edit: StringEdit,
	pub dialog_model: DialogModel,
}

#[derive(Debug, Clone)]
pub enum EditUnspentAction {
	Close,
	Edit(StringEditAction),
	Submit,
	UpdateDialog(DialogAction),
}

#[derive(Debug)]
pub struct Spark {
	pub chad: Chad,
	pub squad_id: u64,
	pub unspent: Option<f64>,
}

impl yui::Spark for Spark {
	type State = State;
	type Action = EditUnspentAction;
	type Report = (u64, f64);

	fn create(&self, ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let string = match self.unspent {
			None => String::new(),
			Some(unspent) => format!("{}", unspent),
		};
		State {
			string_edit: StringEdit::new(string, 0, Validity::Double),
			dialog_model: DialogModel::new(
				ctx.link().to_trigger(EditUnspentAction::Close),
				ctx.link().to_trigger(EditUnspentAction::Submit),
				ctx.link().map(|action| EditUnspentAction::UpdateDialog(action)),
			),
		}
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		let state = ctx.state().clone();
		match action {
			EditUnspentAction::Close => AfterFlow::Close(None),
			EditUnspentAction::Edit(action) => {
				let string_edit = state.string_edit.edit(action);
				let dialog_model = state.dialog_model.enable_submit(state.string_edit.is_valid());
				AfterFlow::Revise(State { string_edit, dialog_model, ..state })
			}
			EditUnspentAction::Submit => {
				let string = state.string_edit.chars.iter().cloned().collect::<String>();
				let unspent = string.parse::<f64>().expect("parse f64");
				self.chad.set_unspent(self.squad_id, unspent);
				AfterFlow::Close(Some((self.squad_id, unspent)))
			}
			EditUnspentAction::UpdateDialog(action) => {
				let dialog_model = state.dialog_model.update(action);
				AfterFlow::Revise(State { dialog_model, ..state })
			}
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let content
			= yard::textfield(UnspentEdit.as_i32(), "Unspent", state.string_edit.clone(), link.map(EditUnspentAction::Edit))
			.confine_height(3, Cling::Top);

		let yard = render::dialog("Set Unspent", &state.dialog_model, content);
		Some(yard)
	}
}