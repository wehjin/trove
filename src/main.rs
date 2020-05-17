extern crate stringedit;
extern crate yui;

use std::error::Error;

use yui::app;

use asset_edit::AssetEdit;

fn main() -> Result<(), Box<dyn Error>> {
	app::run::<AssetEdit>()
}

mod asset_edit {
	use std::collections::HashMap;
	use std::ops::Index;

	use stringedit::StringEdit;
	use yui::{AfterRoll, ArcYard, Cling, Confine, Link, Pack, Padding, RollContext, story, yard};
	use yui::palette::StrokeColor;

	use crate::asset_edit::Field::{Account, Corral, Custodian, Price, Shares, Symbol};

	#[derive(Debug, Clone, Eq, PartialEq)]
	pub struct AssetEdit {
		edits: HashMap<Field, StringEdit>,
	}

	#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
	pub enum Field { Custodian, Account, Symbol, Shares, Corral, Price }

	impl Field {
		pub fn all() -> Vec<Field> { vec![Custodian, Account, Symbol, Shares, Corral, Price] }
	}

	impl AssetEdit {
		pub fn edit(&self, field: Field, action: stringedit::Action) -> Self {
			let edit = self.edits[&field].edit(action);
			let mut edits = self.edits.clone();
			edits.insert(field, edit);
			AssetEdit { edits }
		}
	}

	impl Index<&Field> for AssetEdit {
		type Output = StringEdit;
		fn index(&self, index: &Field) -> &Self::Output { self.edits.get(index).unwrap() }
	}

	pub enum Action {
		Done,
		FieldEdit(Field, stringedit::Action),
	}

	impl story::Wheel for AssetEdit {
		type State = AssetEdit;
		type Action = Action;
		type Report = ();

		fn build(_link: Option<Link<Self::Report>>) -> Self::State {
			let edits = Field::all().into_iter().fold(
				HashMap::new(),
				|mut map, field| {
					map.insert(field, StringEdit::empty());
					map
				},
			);
			AssetEdit { edits }
		}

		fn roll(ctx: &impl RollContext<Self::State, Self::Action>, action: Self::Action) -> AfterRoll<Self::State> {
			match action {
				Action::FieldEdit(field, edit) => {
					let state = ctx.state().edit(field, edit);
					AfterRoll::Revise(state)
				}
				Action::Done => {
					ctx.end_prequel();
					AfterRoll::Ignore
				}
			}
		}

		fn yard(vision: &Self::State, link: &Link<Self::Action>) -> Option<ArcYard> {
			let strands = vec![
				{
					let link = link.to_owned();
					yard::textfield(2000, "Custodian", vision[&Custodian].to_owned(), move |edit| link.send(Action::FieldEdit(Custodian, edit)))
				},
				{
					let link = link.to_owned();
					yard::textfield(2001, "Account", vision[&Account].to_owned(), move |edit| link.send(Action::FieldEdit(Account, edit)))
				},
				{
					let link = link.to_owned();
					yard::textfield(2002, "Symbol", vision[&Symbol].to_owned(), move |edit| link.send(Action::FieldEdit(Symbol, edit)))
				},
				{
					let link = link.to_owned();
					yard::textfield(2003, "Shares", vision[&Shares].to_owned(), move |edit| link.send(Action::FieldEdit(Shares, edit)))
				},
				{
					let link = link.to_owned();
					yard::textfield(2004, "Corral", vision[&Corral].to_owned(), move |edit| link.send(Action::FieldEdit(Corral, edit)))
				},
				{
					let link = link.to_owned();
					yard::textfield(2005, "Price", vision[&Price].to_owned(), move |edit| link.send(Action::FieldEdit(Price, edit)))
				},
			];
			let strand_height = 3;
			let strand_gap = 1;
			let trellis_height = (strand_height + strand_gap) * strands.len() as i32 - 1;
			let trellis = yard::trellis(strand_height, strand_gap, strands);
			let title = yard::title("Add Asset", StrokeColor::BodyOnBackground, Cling::LeftTop);
			let button = {
				let link = link.clone();
				yard::button("Done", move |_| link.send(Action::Done))
			};
			let final_yard = yard::empty()
				.pack_top(3, button.confine(8, 1, Cling::LeftBottom))
				.pack_top(trellis_height, trellis)
				.pack_top(4, title)
				.pad(1);
			Some(final_yard)
		}
	}
}


