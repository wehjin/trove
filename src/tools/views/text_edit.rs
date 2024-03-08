use crate::tools::frame::Frame;

pub struct TextEditor {
	pub text: String,
	pub frame: Frame,
	pub cursor_index: Option<usize>,
	pub left_index: usize,
}

impl TextEditor {
	pub fn new() -> Self {
		Self {
			text: "".to_string(),
			frame: Frame::default(),
			cursor_index: None,
			left_index: 0,
		}
	}
	pub fn with_text(self, text: String) -> Self {
		let new_cursor_index = Self::normalize_cursor_index(&text, self.cursor_index);
		let new_left_index = Self::normalize_left_index(&text, self.frame, new_cursor_index, self.left_index);
		Self {
			text,
			frame: self.frame,
			cursor_index: new_cursor_index,
			left_index: new_left_index,
		}
	}
	pub fn with_frame(self, frame: Frame) -> Self {
		let new_left_index = Self::normalize_left_index(&self.text, frame, self.cursor_index, self.left_index);
		Self {
			text: self.text,
			frame,
			cursor_index: self.cursor_index,
			left_index: new_left_index,
		}
	}

	pub fn with_cursor_index(self, cursor_index: Option<usize>) -> Self {
		let new_cursor_index = Self::normalize_cursor_index(&self.text, cursor_index);
		let new_left_index = Self::normalize_left_index(&self.text, self.frame, new_cursor_index, self.left_index);
		Self {
			text: self.text,
			frame: self.frame,
			cursor_index: new_cursor_index,
			left_index: new_left_index,
		}
	}

	pub fn with_left_index(self, left_index: usize) -> Self {
		let new_left_index = Self::normalize_left_index(&self.text, self.frame, self.cursor_index, left_index);
		Self {
			text: self.text,
			frame: self.frame,
			cursor_index: self.cursor_index,
			left_index: new_left_index,
		}
	}

	fn normalize_cursor_index(text: &String, cursor_index: Option<usize>) -> Option<usize> {
		let text_len = text.chars().count();
		cursor_index.map(|i| i.min(text_len + Self::CURSOR_LEN))
	}
	fn normalize_left_index(text: &String, frame: Frame, cursor_index: Option<usize>, left_index: usize) -> usize {
		let left_index = {
			let text_len = text.chars().count();
			let excess_len = (text_len + Self::CURSOR_LEN) as isize - frame.width() as isize;
			if excess_len <= 0 {
				0
			} else {
				left_index.min(excess_len as usize).min(text_len + Self::CURSOR_LEN)
			}
		};
		if let Some(cursor_index) = cursor_index {
			let min_left_index = (cursor_index as isize - frame.width() as isize + 1).max(0) as usize;
			let max_left_index = (cursor_index as isize + frame.width() as isize - 1).max(0) as usize;
			left_index.max(min_left_index).min(max_left_index)
		} else {
			left_index
		}
	}

	const CURSOR_LEN: usize = 1;
}

