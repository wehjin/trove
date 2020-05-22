extern crate dirs;
extern crate echo_lib;
extern crate stringedit;
extern crate yui;

use std::error::Error;

use yui::app;

use crate::list_assets::ListAssets;

mod asset_edit;
mod data;

#[derive(Debug, Clone)]
pub struct QuadText {
	title: String,
	subtitle: String,
	value: String,
	subvalue: String,
}

impl QuadText {
	pub fn title(&self) -> &String { &self.title }
	pub fn subtitle(&self) -> &String { &self.subtitle }
	pub fn value(&self) -> &String { &self.value }
	pub fn subvalue(&self) -> &String { &self.subvalue }
}

mod list_assets {
	use yui::{AfterFlow, ArcYard, Cling, Confine, Flow, Link, Pack, Padding, Spark, yard};
	use yui::palette::FillColor;

	use crate::asset_edit::EditAsset;
	use crate::QuadText;

	pub struct ListAssets {}

	pub enum Action {
		AddAsset
	}

	impl Spark for ListAssets {
		type State = QuadText;
		type Action = Action;
		type Report = ();

		fn create(&self, _report_link: Option<Link<Self::Report>>) -> Self::State {
			QuadText {
				title: "Amazon, Inc.".into(),
				subtitle: "Focused".into(),
				value: "103 AMZN".into(),
				subvalue: "1 USD".into(),
			}
		}

		fn flow(ctx: &impl Flow<Self::State, Self::Action>, action: Self::Action) -> AfterFlow<Self::State> {
			match action {
				Action::AddAsset => {
					ctx.start_prequel(EditAsset {});
					AfterFlow::Ignore
				}
			}
		}

		fn yard(state: &Self::State, link: &Link<Self::Action>) -> Option<ArcYard> {
			let column_width = 40;
			let item = quad_label(state).pad(1);
			let button = {
				let link = link.to_owned();
				yard::button("Add Asset", move |_| link.send(Action::AddAsset)).pad(1)
			};
			let content = item.confine(column_width, 4, Cling::LeftTop)
				.pack_bottom(5, button.confine_width(column_width, Cling::LeftBottom));
			Some(content)
		}
	}

	fn quad_label(quad_text: &QuadText) -> ArcYard {
		yard::quad_label(
			quad_text.title(),
			quad_text.subtitle(),
			quad_text.value(),
			quad_text.subvalue(),
			15,
			FillColor::Background,
		)
	}
}


fn main() -> Result<(), Box<dyn Error>> {
	app::run(ListAssets {}, None)?;
	Ok(())
}
