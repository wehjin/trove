use yui::{AfterFlow, ArcYard, Cling, Create, Flow, Link, Spark, yard};
use yui::palette::StrokeColor;

#[derive(Debug)]
pub struct ListFactions;

impl ListFactions {
	pub fn new() -> Self { ListFactions }
}

impl Spark for ListFactions {
	type State = ();
	type Action = ();
	type Report = ();

	fn render(_state: &Self::State, _link: &Link<Self::Action>) -> Option<ArcYard> {
		let yard = yard::label("Factions", StrokeColor::CommentOnBackground, Cling::Center);
		Some(yard)
	}

	fn flow(_flow: &impl Flow<Self::State, Self::Action, Self::Report>, _action: Self::Action) -> AfterFlow<Self::State, Self::Report> {
		AfterFlow::Ignore
	}

	fn create(&self, _create: &Create<Self::Action, Self::Report>) -> Self::State {
		()
	}
}