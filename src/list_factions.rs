use chad_core::core::SegmentType;
use chad_core::portfolio::Portfolio;
use chad_core::portfolio::segment::Segment;
use chad_core::storage_link::StorageLink;
use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, Pack, Padding, SenderLink, Spark, yard};
use yui::palette::StrokeColor;
use yui::yard::Pressable;

use crate::YardId;

#[derive(Debug)]
pub struct ListFactions { pub link: StorageLink }

impl Spark for ListFactions {
	type State = Vec<Segment>;
	type Action = ();
	type Report = ();

	fn create(&self, _create: &Create<Self::Action, Self::Report>) -> Self::State {
		self.link.latest_portfolio().segments()
	}

	fn flow(&self, _action: Self::Action, _flow: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		AfterFlow::Ignore
	}

	fn render(state: &Self::State, _link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let items = state.iter().map(|segment| {
			let title = yard::label(
				&segment_name(segment),
				if SegmentType::Unknown == segment.segment_type() { StrokeColor::EnabledOnBackground } else { StrokeColor::BodyOnBackground },
				Cling::Left,
			);
			let subtitle = yard::label(&market_value(segment), StrokeColor::CommentOnBackground, Cling::Right);
			let content = title.pack_bottom(1, subtitle);
			(4, content.pad(1).pressable(SenderLink::ignore()))
		}).collect::<Vec<_>>();
		let list = yard::list(YardId::FactionsList.as_i32(), 0, items);
		Some(list.confine_width(40, Cling::Center))
	}
}

fn segment_name(segment: &Segment) -> String {
	match segment.segment_type() {
		SegmentType::Liquid => "Cash",
		SegmentType::Stable => "Coin",
		SegmentType::Linear => "Income",
		SegmentType::Expo => "Growth",
		SegmentType::Unknown => "Unassigned",
	}.to_string()
}

fn market_value(segment: &Segment) -> String {
	format!("{} USD", segment.segment_value())
}