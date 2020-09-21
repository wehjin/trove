use std::collections::HashMap;
use std::ops::Index;

use chad_core::core::{AssetCode, LotId};
use chad_core::portfolio::lot::Lot;
use stringedit::{StringEdit, Validity};
use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, Pack, Padding, SenderLink, story, yard};
use yui::palette::StrokeColor;
use yui::yard::ButtonState;

use crate::edit_lot::Field::{Account, Corral, Custodian, Price, Shares, Symbol};

#[derive(Clone)]
pub struct State {
	lot_id: LotId,
	edits: HashMap<Field, StringEdit>,
	active_field: Field,
}

pub enum Action {
	Done(Option<Lot>),
	FieldEdit(Field, stringedit::Action),
}

pub struct EditLot {
	pub lot_id: LotId,
	pub start_values: Vec<(Field, String)>,
}

impl story::Spark for EditLot {
	type State = State;
	type Action = Action;
	type Report = Lot;

	fn create(&self, _create: &Create<Self::Action, Self::Report>) -> Self::State {
		let mut start_values = HashMap::new();
		self.start_values.iter().for_each(|(field, value)| {
			start_values.insert(*field, value.to_owned());
		});
		let edits = Field::all().into_iter().fold(
			HashMap::new(),
			|mut map, field| {
				let edit = match start_values.get(&field) {
					None => StringEdit::empty(field.validity()),
					Some(v) => StringEdit::new(v.to_owned(), 0, field.validity()),
				};
				map.insert(field, edit);
				map
			},
		);
		State { lot_id: self.lot_id, edits, active_field: Field::Custodian }
	}

	fn flow(&self, action: Self::Action, flow: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			Action::FieldEdit(field, edit) => {
				let state = flow.state().edit(field, edit);
				AfterFlow::Revise(state)
			}
			Action::Done(lot) => {
				if let Some(lot) = lot { flow.report(lot) }
				flow.end_prequel();
				AfterFlow::Ignore
			}
		}
	}

	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let text_fields = vec![
			yard::textfield(
				2000,
				"Custodian*",
				state[&Custodian].to_owned(),
				link.map(|edit| Action::FieldEdit(Custodian, edit)),
			),
			yard::textfield(
				2001,
				"Account*",
				state[&Account].to_owned(),
				link.map(|edit| Action::FieldEdit(Account, edit)),
			),
			yard::textfield(2002, "Symbol*", state[&Symbol].to_owned(), link.map(|edit| Action::FieldEdit(Symbol, edit))),
			yard::textfield(2003, "Shares*", state[&Shares].to_owned(), link.map(|edit| Action::FieldEdit(Shares, edit))),
			yard::textfield(2004, "Corral", state[&Corral].to_owned(), link.map(|edit| Action::FieldEdit(Corral, edit))),
			yard::textfield(2005, "Price", state[&Price].to_owned(), link.map(|edit| Action::FieldEdit(Price, edit))),
			{
				let label = "Add   ";
				let button = if let Some(lot) = state.completed_lot() {
					let on_click = link.map(move |_| Action::Done(Some(lot.clone())));
					yard::button(label, ButtonState::enabled(on_click))
				} else {
					yard::button(label, ButtonState::disabled())
				};
				button.pad(1).confine_height(5, Cling::Top)
			},
		];
		let items = text_fields.into_iter().map(|it| {
			(4u8, it.confine_height(3, Cling::Top))
		}).collect();
		let list = yard::list(2999, state.active_field.rank(), items);
		let title = yard::title("Add Lot", StrokeColor::BodyOnBackground, Cling::LeftTop);
		let cancel = yard::button("Cancel", ButtonState::enabled(link.map(|_| Action::Done(None))))
			.confine_height(3, Cling::Top);
		let yard = list
			.pack_right(20, cancel.pad_cols(1))
			.pack_top(3, title)
			.pad(1);
		Some(yard)
	}
}


impl State {
	pub fn edit(&self, field: Field, action: stringedit::Action) -> Self {
		let edit = self.edits[&field].edit(action);
		let mut edits = self.edits.clone();
		edits.insert(field, edit);
		State { lot_id: self.lot_id, edits, active_field: field }
	}
	pub fn completed_lot(&self) -> Option<Lot> {
		if self.is_ready_for_save() {
			let lot = Lot {
				lot_id: self.lot_id,
				asset_code: AssetCode::Common(self.edits[&Field::Symbol].read().to_uppercase()),
				share_count: self.edits[&Field::Shares].read().trim().parse::<f64>().unwrap(),
				custodian: chad_core::core::Custodian::Custom(self.edits[&Field::Custodian].read()),
				account: chad_core::core::Account::Main,
			};
			Some(lot)
		} else {
			None
		}
	}
	pub fn is_ready_for_save(&self) -> bool {
		let mut complete = true;
		for (_key, value) in &self.edits {
			if !value.is_valid() {
				complete = false;
				break;
			}
		}
		complete
	}
}

impl Index<&Field> for State {
	type Output = StringEdit;
	fn index(&self, index: &Field) -> &Self::Output { self.edits.get(index).unwrap() }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Field { Custodian, Account, Symbol, Shares, Corral, Price }

impl Field {
	pub fn validity(&self) -> Validity {
		match self {
			Custodian => Validity::NotEmpty,
			Account => Validity::NotEmpty,
			Symbol => Validity::NotEmpty,
			Shares => Validity::UnsignedInt,
			Corral => Validity::NotEmpty,
			Price => Validity::Double,
		}
	}
	pub fn rank(&self) -> usize {
		match &self {
			Custodian => 0,
			Account => 1,
			Symbol => 2,
			Shares => 3,
			Corral => 4,
			Price => 5,
		}
	}
}

impl Field {
	pub fn all() -> Vec<Field> { vec![Custodian, Account, Symbol, Shares, Corral, Price] }
}
