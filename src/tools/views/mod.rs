use std::ops::Add;

use crate::tools::captor::Captor;
use crate::tools::fill::Fill;
use crate::tools::frame::Frame;

pub mod fab;
pub mod scroll_list;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ZMax(i16);

impl ZMax {
	pub fn z(&self) -> i16 { self.0 }
}

impl Add<usize> for ZMax {
	type Output = ZMax;

	fn add(self, rhs: usize) -> Self::Output { ZMax(self.0 + rhs as i16) }
}

pub trait EdgeHolder {
	fn set_edge(&mut self, frame: Frame) -> ZMax;
}

pub trait View {
	type Msg: Send + Sync + 'static;
	fn update(&mut self, msg: Self::Msg);
	fn get_fills_captors(&self, edge_frame: Frame) -> (Vec<Fill>, Vec<Captor<Self::Msg>>);
}
