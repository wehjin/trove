use std::collections::HashMap;
use std::ops::Index;

use stringedit::{StringEdit, Validity};
use yui::{AfterFlow, ArcYard, Cling, Confine, Flow, Link, Pack, Padding, story, yard};
use yui::palette::StrokeColor;

use crate::data::Asset;
use crate::edit_asset::Field::{Account, Corral, Custodian, Price, Shares, Symbol};

#[derive(Debug, Clone)]
pub enum Report {
	Changed(Asset),
}

pub struct EditAsset;

impl story::Spark for EditAsset {
	type State = State;
	type Action = Action;
	type Report = Report;

	fn create(&self, report_link: Option<Link<Self::Report>>) -> Self::State {
		let edits = Field::all().into_iter().fold(
			HashMap::new(),
			|mut map, field| {
				map.insert(field, StringEdit::empty(field.validity()));
				map
			},
		);
		State { edits, active_field: Field::Custodian, report_link }
	}

	fn flow(ctx: &impl Flow<Self::State, Self::Action>, action: Self::Action) -> AfterFlow<Self::State> {
		match action {
			Action::FieldEdit(field, edit) => {
				let state = ctx.state().edit(field, edit);
				AfterFlow::Revise(state)
			}
			Action::Done(asset) => {
				if let Some(asset) = asset {
					match &ctx.state().report_link {
						Some(link) => link.send(Report::Changed(asset)),
						_ => {}
					}
				}
				ctx.end_prequel();
				AfterFlow::Ignore
			}
		}
	}

	fn yard(state: &Self::State, link: &Link<Self::Action>) -> Option<ArcYard> {
		let text_fields = vec![
			{
				let link = link.to_owned();
				yard::textfield(2000, "Custodian*", state[&Custodian].to_owned(), move |edit| {
					link.send(Action::FieldEdit(Custodian, edit))
				})
			},
			{
				let link = link.to_owned();
				yard::textfield(2001, "Account*", state[&Account].to_owned(), move |edit| {
					link.send(Action::FieldEdit(Account, edit))
				})
			},
			{
				let link = link.to_owned();
				yard::textfield(2002, "Symbol*", state[&Symbol].to_owned(), move |edit| link.send(Action::FieldEdit(Symbol, edit)))
			},
			{
				let link = link.to_owned();
				yard::textfield(2003, "Shares*", state[&Shares].to_owned(), move |edit| link.send(Action::FieldEdit(Shares, edit)))
			},
			{
				let link = link.to_owned();
				yard::textfield(2004, "Corral", state[&Corral].to_owned(), move |edit| link.send(Action::FieldEdit(Corral, edit)))
			},
			{
				let link = link.to_owned();
				yard::textfield(2005, "Price", state[&Price].to_owned(), move |edit| link.send(Action::FieldEdit(Price, edit)))
			},
			{
				let label = "Add   ";
				let button = match state.completed_asset() {
					Some(asset) => {
						let link = link.clone();
						yard::button_enabled(label, move |_| link.send(Action::Done(Some(asset.clone()))))
					}
					None => yard::button_disabled(label),
				};
				button.pad(1).confine_height(5, Cling::Top)
			},
		];
		let items = text_fields.into_iter().map(|it| {
			(4u8, it.confine_height(3, Cling::Top))
		}).collect();
		let list = yard::list(2999, state.active_field.rank(), items);
		let title = yard::title("Add Asset", StrokeColor::BodyOnBackground, Cling::LeftTop);
		let cancel = {
			let link = link.clone();
			let button = yard::button_enabled("Cancel", move |_| link.send(Action::Done(None)));
			button.confine_height(3, Cling::Top)
		};

		let yard = list
			.pack_right(20, cancel.pad_cols(1))
			.pack_top(3, title)
			.pad(1);
		Some(yard)
	}
}

#[derive(Clone)]
pub struct State {
	edits: HashMap<Field, StringEdit>,
	active_field: Field,
	report_link: Option<Link<Report>>,
}

impl State {
	pub fn edit(&self, field: Field, action: stringedit::Action) -> Self {
		let edit = self.edits[&field].edit(action);
		let mut edits = self.edits.clone();
		edits.insert(field, edit);
		State {
			edits,
			active_field: field,
			report_link: self.report_link.to_owned(),
		}
	}
	pub fn completed_asset(&self) -> Option<Asset> {
		if self.is_ready_for_save() {
			let asset = Asset::new(
				&self.edits[&Field::Symbol].read(),
				&self.edits[&Field::Account].read(),
				&self.edits[&Field::Custodian].read(),
				&self.edits[&Field::Corral].read(),
				self.edits[&Field::Shares].read().trim().parse::<u64>().unwrap(),
			);
			Some(asset)
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

pub enum Action {
	Done(Option<Asset>),
	FieldEdit(Field, stringedit::Action),
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
