use echo_lib::Echo;
use yui::{AfterFlow, ArcYard, Cling, Confine, Create, Flow, Link, Pack, Padding, Spark, yard};
use yui::palette::StrokeColor;
use yui::yard::Pressable;

use crate::data::{Asset, read_factions};
use crate::YardId;

#[derive(Debug)]
pub struct ListFactions { echo: Echo }

impl ListFactions {
	pub fn new(echo: &Echo) -> Self { ListFactions { echo: echo.clone() } }
}

impl Spark for ListFactions {
	type State = Echo;
	type Action = ();
	type Report = ();

	fn render(state: &Self::State, _link: &Link<Self::Action>) -> Option<ArcYard> {
		let yard_result = state.chamber()
			.and_then(|mut chamber| read_factions(&mut chamber))
			.map(|factions| {
				let items = factions.iter().map(|it| {
					let title_line = yard::label(it.name().to_lowercase(), StrokeColor::BodyOnBackground, Cling::Left);
					let subtitle_line = yard::label(&asset_count_string(&it.assets), StrokeColor::CommentOnBackground, Cling::Left);
					let yard = title_line.pack_bottom(1, subtitle_line);
					(4, yard.pad(1).pressable(|_| {}))
				}).collect::<Vec<_>>();
				items
			})
			.map(|items| yard::list(YardId::FactionsList.as_i32(), 0, items));
		let yard = match yard_result {
			Ok(yard) => yard.confine_width(40, Cling::Center),
			Err(e) => yard::label(&format!("Error: {}", e.to_string()), StrokeColor::CommentOnBackground, Cling::Center),
		};
		Some(yard)
	}

	fn flow(_flow: &impl Flow<Self::State, Self::Action, Self::Report>, _action: Self::Action) -> AfterFlow<Self::State, Self::Report> {
		AfterFlow::Ignore
	}

	fn create(&self, _create: &Create<Self::Action, Self::Report>) -> Self::State { self.echo.clone() }
}

fn asset_count_string(assets: &Vec<Asset>) -> String {
	let asset_count = assets.len();
	if asset_count == 1 {
		"1 asset".to_string()
	} else {
		format!("{} assets", asset_count)
	}
}