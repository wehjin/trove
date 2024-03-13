use std::ops::Add;

use crate::tools::captor::{Captor, CaptorId};
use crate::tools::fill::Fill;
use crate::tools::frame::Frame;

pub mod fab;
pub mod scroll_list;
pub mod text_edit;

#[must_use]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ZMax(pub i16);

impl ZMax {
	pub fn z(&self) -> i16 { self.0 }
	pub fn max(self, other: ZMax) -> Self { ZMax(self.0.max(other.0)) }
}

impl Add<usize> for ZMax {
	type Output = ZMax;

	fn add(self, rhs: usize) -> Self::Output { ZMax(self.0 + rhs as i16) }
}

pub trait Updating {
	type Msg;
	fn update(&mut self, msg: Self::Msg);
}

pub trait Shaping {
	fn shape(&mut self, edge_frame: Frame) -> ZMax;
}

pub trait Viewing: Shaping {
	type Msg;
	fn get_fills_captors(&self, active_captor_id: Option<CaptorId>) -> (Vec<Fill>, Vec<Captor<Self::Msg>>);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CursorEvent {
	Focus,
	Select,
	MoveLeft,
	MoveRight,
	DeleteBack,
	Char(char),
}