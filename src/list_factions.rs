use yui::{AfterFlow, Create, Flow, Spark};

pub struct ListFactions;

impl ListFactions {
	pub fn new() -> Self { ListFactions }
}

impl Spark for ListFactions {
	type State = ();
	type Action = ();
	type Report = ();

	fn flow(_flow: &impl Flow<Self::State, Self::Action, Self::Report>, _action: Self::Action) -> AfterFlow<Self::State, Self::Report> {
		AfterFlow::Ignore
	}

	fn create(&self, _create: &Create<Self::Action, Self::Report>) -> Self::State {
		()
	}
}