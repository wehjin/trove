use std::collections::{BTreeMap, HashMap};

use crossterm::style::Color;

use crate::tools::console::Console;

#[derive(Default)]
pub struct Screen {
	max_col: u16,
	max_row: u16,
	rooms: HashMap<(u16, u16), Room>,
}

impl Screen {
	pub fn new((cols, rows): (u16, u16)) -> Self {
		let mut screen = Self { max_col: cols, max_row: rows, ..Screen::default() };
		for row in 0..screen.max_row {
			for col in 0..screen.max_col {
				let rune = match (row & 1) == 0 {
					true => Rune('k', Color::Yellow),
					false => Rune('z', Color::Green),
				};
				screen.room_mut(col, row).add_rune(1, rune);
				screen.room_mut(col, row).add_tile(0, Tile(Color::Black));
			}
		}
		screen
	}
	pub fn room(&self, col: u16, row: u16) -> Option<&Room> {
		self.rooms.get(&(col, row))
	}
	pub fn room_mut(&mut self, col: u16, row: u16) -> &mut Room {
		if self.rooms.get(&(col, row)).is_none() {
			self.rooms.insert((col, row), Room::default());
		}
		self.rooms.get_mut(&(col, row)).expect("room")
	}
	pub fn print(&self, console: &mut Console) {
		for row in 0..self.max_row {
			for col in 0..self.max_col {
				if let Some(room) = self.room(col, row) {
					let rune = room.top_rune();
					let tile = room.top_tile();
					console.move_color_print(col, row, rune.0, rune.1, tile.0);
				}
			}
		}
		console.flush();
	}
}

#[derive(Default)]
pub struct Room {
	z_tiles: BTreeMap<i16, Tile>,
	z_runes: BTreeMap<i16, Rune>,
}

impl Room {
	pub fn add_tile(&mut self, z: u16, tile: Tile) {
		let z: i16 = -(z as i16);
		self.z_tiles.insert(z, tile);
	}
	pub fn add_rune(&mut self, z: u16, rune: Rune) {
		let z: i16 = -(z as i16);
		self.z_runes.insert(z, rune);
	}
	pub fn top_tile(&self) -> Tile {
		if self.z_tiles.is_empty() {
			Tile(Color::Magenta)
		} else {
			let top_key = self.z_tiles.keys().next().expect("tile key");
			self.z_tiles.get(top_key).cloned().expect("tile")
		}
	}
	pub fn top_rune(&self) -> Rune {
		if self.z_runes.is_empty() {
			Rune('?', Color::Magenta)
		} else {
			let top_key = self.z_runes.keys().next().expect("rune key");
			self.z_runes.get(top_key).cloned().expect("rune")
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tile(pub Color);

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rune(pub char, pub Color);

