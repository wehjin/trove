use crate::tools::fill::Fill;
use crate::tools::frame::Frame;

pub mod fab;

pub trait ViewStarting {
	type Model: ViewChanging;

	fn init(self) -> Self::Model;
}

pub trait ViewChanging {
	type Msg: Send + Sync + 'static;
	fn update(&mut self, msg: Self::Msg);
}

pub trait View {
	fn get_fills(&self, edge_frame: Frame) -> Vec<Fill>;
}
