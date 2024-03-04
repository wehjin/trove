use std::collections::HashMap;

use crate::tools::frame::Frame;
use crate::tools::inset::Inset;

pub struct Layout {
	sealed: HashMap<String, Frame>,
	todo: Vec<Frame>,
}

impl Layout {
	pub fn new(frame: Frame) -> Self {
		Layout {
			sealed: HashMap::new(),
			todo: vec![frame],
		}
	}
}

impl Layout {
	pub fn inset(mut self, inset: Inset) -> Self {
		let frame = self.todo.pop().expect("current frame");
		self.todo.push(frame.inset(inset));
		self
	}
	pub fn move_closer(mut self, layers: u16) -> Self {
		let frame = self.todo.pop().expect("current frame");
		self.todo.push(frame.move_closer(layers));
		self
	}
	pub fn split_top(mut self, rows: u16) -> Self {
		let frame = self.todo.pop().expect("current frame");
		let (top, bottom) = frame.split_from_top(rows);
		self.todo.push(bottom);
		self.todo.push(top);
		self
	}
	pub fn take(mut self, out: &mut Frame) -> Self {
		let frame = self.todo.pop().expect("current frame");
		*out = frame;
		self
	}

	pub fn seal(mut self, label: impl AsRef<str>) -> Self {
		let frame = self.todo.pop().expect("current frame");
		let label = label.as_ref().to_string();
		self.sealed.insert(label, frame);
		self
	}
	pub fn into_frames(self) -> HashMap<String, Frame> {
		assert_eq!(0, self.todo.len());
		self.sealed
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
		let frames = Layout::new(frame)
			.inset(Inset::Cols(1))
			.seal("a")
			.into_frames()
			;
		assert_eq!(1, frames["a"].left);
	}

	#[test]
	fn branching() {
		let frame = Frame::from_cols_rows_z(100, 50, 1);
		let out = Layout::new(frame)
			.split_top(10)
			.inset(Inset::DoubleCols(1)).seal("header")
			.seal("body")
			.into_frames()
			;
		assert_eq!(1, out["header"].top);
		assert_eq!(10, out["body"].top);
	}
}

