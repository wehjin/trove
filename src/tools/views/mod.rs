use crate::tools::captor::Captor;
use crate::tools::fill::Fill;
use crate::tools::frame::Frame;

pub mod fab;
pub trait View {
	type Msg: Send + Sync + 'static;
	fn update(&mut self, msg: Self::Msg);
	fn get_fills_captors(&self, edge_frame: Frame) -> (Vec<Fill>, Vec<Captor<Self::Msg>>);
}
