use yui::{AfterFlow, ArcYard, Before, Cling, Create, Flow, Link, Pack, Padding, Spark, yard};
use yui::palette::{FillColor, StrokeColor};
use yui::yard::Pressable;

use crate::data::{Asset, Lot};

pub struct ViewAsset {
	pub asset: Asset,
}

pub enum Action {
	Close
}

impl Spark for ViewAsset {
	type State = Asset;
	type Action = Action;
	type Report = ();

	fn yard(state: &Self::State, link: &Link<Self::Action>) -> Option<ArcYard> {
		let title = format!("{}", state.symbol).to_uppercase();
		let detail1 = format!("{} {}", state.shares(), state.symbol.to_lowercase());
		let detail2 = format!("$???");
		let detail3 = format!("{}", state.corral);
		let banner_content = yard::trellis(1, 0, vec![
			yard::label(&title, StrokeColor::BodyOnPrimary, Cling::Left),
			yard::label(&detail1, StrokeColor::CommentOnPrimary, Cling::Left),
			yard::label(&detail2, StrokeColor::CommentOnPrimary, Cling::Left),
			yard::label(&detail3, StrokeColor::CommentOnPrimary, Cling::Left),
		]);
		let banner = banner_content
			.pad(1)
			.before(yard::fill(FillColor::Primary));
		let lot_items = state.lots.iter().map(Lot::to_list_item).collect();
		let lot_list = yard::list(LIST_ID, 0, lot_items);
		let close_button = {
			let link = link.clone();
			yard::button_enabled("Close", move |_| link.send(Action::Close))
		};
		let content = lot_list
			.pack_top(6, banner)
			.pack_bottom(3, close_button);
		Some(content)
	}

	fn flow(flow: &impl Flow<Self::State, Self::Action, Self::Report>, action: Self::Action) -> AfterFlow<Self::State> {
		match action {
			Action::Close => {
				flow.end_prequel();
				AfterFlow::Ignore
			}
		}
	}

	fn create(&self, _create: &Create<Self::Action, Self::Report>) -> Self::State {
		self.asset.clone()
	}
}

impl Lot {
	fn to_list_item(&self) -> (u8, ArcYard) {
		let title = format!("{}/{}", self.custodian(), self.account());
		let subtitle = format!("{} shares", self.shares());
		let detail = format!("$???");
		let content = yard::trellis(1, 0, vec![
			yard::label(&title, StrokeColor::BodyOnBackground, Cling::Left),
			yard::label(&subtitle, StrokeColor::CommentOnBackground, Cling::Left),
			yard::label(&detail, StrokeColor::CommentOnBackground, Cling::Left),
		]);
		(5, content.pad(1).pressable(|_| {}))
	}
}

const LIST_ID: i32 = 11000;