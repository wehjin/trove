use crossbeam::channel::Sender;
use rand::random;

use crate::tools::{Cmd, solar_dark};
use crate::tools::beats::{Beat, signal};
use crate::tools::captor::{Captor, CaptorId, CaptorKind};
use crate::tools::fill::{Fill, string_to_fills};
use crate::tools::frame::Frame;
use crate::tools::frame::layout::Layout;
use crate::tools::inset::Inset;
use crate::tools::views::{CursorEvent, Shaping, Updating, Viewing, ZMax};

#[derive(Debug, Clone)]
pub enum TextEditorMsg {
	SetText(String),
	SetEdge(Frame),
	OnCursor(CursorEvent),
}

pub struct TextEditor {
	pub id: u64,
	pub text: String,
	pub edge: Frame,
	pub cursor_index: usize,
	pub left_index: usize,
	pub text_frame: Frame,
	cursor_events_sender: Sender<CursorEvent>,
	cursor_events_beat: Beat<TextEditorMsg>,
}

impl TextEditor {
	pub fn new() -> Self {
		let (on_cursor_event, cursor_beat) = signal(TextEditorMsg::OnCursor);
		Self {
			id: random(),
			text: "".to_string(),
			edge: Frame::default(),
			cursor_index: 0,
			left_index: 0,
			text_frame: Frame::default(),
			cursor_events_sender: on_cursor_event,
			cursor_events_beat: cursor_beat,
		}
	}
	fn to_frames(&self, frame: Frame, head: &str) -> (Frame, Frame, Frame) {
		let head_len = head.chars().count();
		let head_frame = frame.with_width_from_left(head_len as u16);
		let cursor_frame = frame.inset(Inset::Left(head_len as u8))
			.with_width_from_left(Self::CURSOR_LEN as u16);
		let head_cursor_len = head_len + Self::CURSOR_LEN;
		let tail_frame = frame.inset(Inset::Left(head_cursor_len as u8));
		(head_frame, cursor_frame, tail_frame)
	}

	fn captor_id(&self) -> CaptorId {
		let id = CaptorId(self.id, 0);
		id
	}
}

impl Default for TextEditor {
	fn default() -> Self { Self::new() }
}

impl Updating for TextEditor {
	type Msg = TextEditorMsg;

	fn update(&mut self, msg: Self::Msg) -> Cmd<Self::Msg> {
		match msg {
			TextEditorMsg::SetText(text) => self.set_text(text),
			TextEditorMsg::SetEdge(edge) => self.set_edge(edge),
			TextEditorMsg::OnCursor(event) => {
				match event {
					CursorEvent::Focus => self.set_cursor_index(self.text.len()),
					CursorEvent::Select => self.insert_char(' '),
					CursorEvent::MoveLeft => self.move_cursor_left(),
					CursorEvent::MoveRight => self.move_cursor_right(),
					CursorEvent::DeleteBack => self.delete_back(),
					CursorEvent::Char(c) => self.insert_char(c),
				}
			}
		}
		Cmd::None
	}
	fn get_beats(&self) -> Vec<Beat<Self::Msg>> {
		vec![self.cursor_events_beat.clone()]
	}
}

impl Shaping for TextEditor {
	fn shape(&mut self, edge_frame: Frame) -> ZMax {
		self.set_edge(edge_frame);
		Layout::new(edge_frame)
			.move_closer(1)
			.take(&mut self.text_frame)
			.into_z_max()
	}
}

impl Viewing for TextEditor {
	type Msg = TextEditorMsg;

	fn get_fills_captors(&self, active_captor_id: Option<CaptorId>) -> (Vec<Fill>, Vec<Captor<Self::Msg>>) {
		let head = &self.text[self.left_index..self.cursor_index];
		let text_len = self.text.chars().count();
		let (cursor, tail) = if self.cursor_index == text_len {
			(" ", "")
		} else {
			let cursor_end = self.cursor_index + Self::CURSOR_LEN;
			let right_index = self.left_index + self.text_frame.width() as usize;
			let tail = if right_index <= cursor_end || right_index >= text_len {
				""
			} else {
				&self.text[cursor_end..right_index]
			};
			(&self.text[self.cursor_index..cursor_end], tail)
		};
		let back_fills = {
			let cursor_color = if active_captor_id == Some(self.captor_id()) {
				solar_dark::BASE3
			} else {
				solar_dark::BASE01
			};
			let (head_frame, cursor_frame, tail_frame) = self.to_frames(self.edge, head);
			let head_fill = Fill::color_tile(head_frame, solar_dark::BASE03);
			let cursor_fill = Fill::color_tile(cursor_frame, cursor_color);
			let tail_fill = Fill::color_tile(tail_frame, solar_dark::BASE03);
			vec![head_fill, cursor_fill, tail_fill]
		};
		let text_fills = {
			let cursor_color = if active_captor_id == Some(self.captor_id()) {
				solar_dark::BASE00
			} else {
				solar_dark::BASE2
			};
			let (head_frame, cursor_frame, tail_frame) = self.to_frames(self.text_frame, head);
			let head_fills = string_to_fills(head, head_frame, solar_dark::BASE0);
			let cursor_fills = string_to_fills(cursor, cursor_frame, cursor_color);
			let tail_fills = string_to_fills(tail, tail_frame, solar_dark::BASE0);
			vec![head_fills, cursor_fills, tail_fills].into_iter().flatten().collect::<Vec<_>>()
		};
		let captor = Captor {
			id: self.captor_id(),
			kind: CaptorKind {
				takes_chars: true,
				takes_delete_back: self.cursor_index > 0,
				takes_select: true,
			},
			cursor_events_sender: Some(self.cursor_events_sender.clone()),
			frame: self.text_frame,
			pre_focus_msg: TextEditorMsg::OnCursor(CursorEvent::Focus),
		};
		let fills = vec![back_fills, text_fills].into_iter().flatten().collect();
		(fills, vec![captor])
	}
}

impl TextEditor {
	fn delete_back(&mut self) {
		if self.cursor_index > 0 {
			let target_index = self.cursor_index - 1;
			let head = &self.text[..target_index];
			let tail = &self.text[self.cursor_index..];
			let text = format!("{head}{tail}");
			self.set_text(text);
			if self.cursor_index != target_index {
				self.set_cursor_index(target_index);
			}
		}
	}
	fn move_cursor_left(&mut self) {
		if self.cursor_index > 0 {
			self.set_cursor_index(self.cursor_index - 1)
		}
	}
	fn move_cursor_right(&mut self) {
		let text_len = self.text.chars().count();
		if self.cursor_index < text_len {
			self.set_cursor_index(self.cursor_index + 1);
		}
	}
	fn insert_char(&mut self, char: char) {
		let head = &self.text[..self.cursor_index];
		let tail = &self.text[self.cursor_index..];
		let text = format!("{head}{char}{tail}");
		self.set_text(text);
		self.set_cursor_index(self.cursor_index + 1);
	}
	fn set_text(&mut self, text: String) {
		let new_cursor_index = Self::normalize_cursor_index(&text, self.cursor_index);
		let new_left_index = Self::normalize_left_index(&text, self.edge, new_cursor_index, self.left_index);
		self.text = text;
		self.cursor_index = new_cursor_index;
		self.left_index = new_left_index;
	}
	fn set_edge(&mut self, edge: Frame) {
		let new_left_index = Self::normalize_left_index(&self.text, edge, self.cursor_index, self.left_index);
		self.edge = edge;
		self.left_index = new_left_index;
	}
	fn set_cursor_index(&mut self, cursor_index: usize) {
		let new_cursor_index = Self::normalize_cursor_index(&self.text, cursor_index);
		let new_left_index = Self::normalize_left_index(&self.text, self.edge, new_cursor_index, self.left_index);
		self.cursor_index = new_cursor_index;
		self.left_index = new_left_index;
	}
	fn normalize_cursor_index(text: &String, cursor_index: usize) -> usize {
		let text_len = text.chars().count();
		cursor_index.min(text_len)
	}
	fn normalize_left_index(text: &String, frame: Frame, cursor_index: usize, left_index: usize) -> usize {
		let left_index = {
			let text_len = text.chars().count();
			let excess_len = (text_len + Self::CURSOR_LEN) as isize - frame.width() as isize;
			if excess_len <= 0 {
				0
			} else {
				left_index.min(excess_len as usize).min(text_len + Self::CURSOR_LEN)
			}
		};
		let min_left_index = (cursor_index as isize - frame.width() as isize + 1).max(0) as usize;
		let max_left_index = (cursor_index as isize + frame.width() as isize - 1).max(0) as usize;
		left_index.max(min_left_index).min(max_left_index)
	}
	const CURSOR_LEN: usize = 1;
}

