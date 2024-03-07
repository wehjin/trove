use crate::tools::frame::Frame;
use crate::tools::inset::Inset;
use crate::tools::views::{Shaper, ZMax};

pub struct Layout {
	todo: Vec<Frame>,
	z_max: ZMax,
}

impl Layout {
	pub fn new(frame: Frame) -> Self {
		let z_max = ZMax(frame.z);
		Layout {
			todo: vec![frame],
			z_max,
		}
	}
	pub fn into_z_max(self) -> ZMax { self.z_max }
}

impl Layout {
	pub fn inset(mut self, inset: Inset) -> Self {
		let frame = self.pop_todo();
		self.todo.push(frame.inset(inset));
		self
	}

	pub fn move_closer(mut self, layers: u16) -> Self {
		let frame = self.pop_todo();
		self.todo.push(frame.move_closer(layers));
		self
	}
	pub fn split_top(mut self, rows: u16) -> Self {
		let frame = self.pop_todo();
		let (top, bottom) = frame.split_from_top(rows);
		self.todo.push(bottom);
		self.todo.push(top);
		self
	}
	pub fn split_right(mut self, cols: u16) -> Self {
		let frame = self.pop_todo();
		let (right, left) = frame.split_from_right(cols);
		self.todo.push(left);
		self.todo.push(right);
		self
	}
	pub fn shape(mut self, shaper: &mut impl Shaper) -> Self {
		let frame = self.pop_todo();
		self.z_max = self.z_max.max(shaper.shape(frame));
		self.todo.push(frame);
		self
	}
	pub fn tag(mut self, out: &mut Frame) -> Self {
		let frame = self.pop_todo();
		self.z_max = self.z_max.max(ZMax(frame.z));
		*out = frame;
		self.todo.push(frame);
		self
	}
	pub fn take(mut self, out: &mut Frame) -> Self {
		let frame = self.pop_todo();
		self.z_max = self.z_max.max(ZMax(frame.z));
		*out = frame;
		self
	}
	pub fn seal(mut self) -> Self {
		let frame = self.pop_todo();
		self.z_max = self.z_max.max(ZMax(frame.z));
		self
	}
	fn pop_todo(&mut self) -> Frame {
		let frame = self.todo.pop().expect("current frame");
		frame
	}
}

#[cfg(test)]
mod tests {
	use crate::tools::frame::Frame;
	use crate::tools::frame::layout::Layout;
	use crate::tools::inset::Inset;

	#[test]
	fn unary() {
		let frame = Frame::from_cols_rows_z(100, 50, 1);
		let mut a_frame = Frame::default();
		Layout::new(frame)
			.inset(Inset::Cols(1))
			.take(&mut a_frame)
		;
		assert_eq!(1, a_frame.left);
	}

	#[test]
	fn branching() {
		let frame = Frame::from_cols_rows_z(100, 50, 1);
		let mut header_frame = Frame::default();
		let mut body_frame = Frame::default();
		Layout::new(frame)
			.split_top(10)
			.inset(Inset::DoubleCols(1)).take(&mut header_frame)
			.take(&mut body_frame)
		;
		assert_eq!(1, header_frame.top);
		assert_eq!(10, body_frame.top);
	}
}

