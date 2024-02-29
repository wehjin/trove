use std::collections::{BTreeMap, HashMap};

use crossterm::style::Color;

use crate::tools::console::Console;
use crate::tools::fill::{Fill, Glyph};
use crate::tools::solar_dark;

#[derive(Default)]
pub struct Screen {
	max_col: u16,
	max_row: u16,
	rooms: HashMap<(u16, u16), Room>,
}

impl Screen {
	pub fn in_range_col_row(&self, col: i16, row: i16) -> Option<(u16, u16)> {
		if col >= 0 && (col as u16) < self.max_col
			&& row >= 0 && (row as u16) < self.max_row {
			Some((col as u16, row as u16))
		} else {
			None
		}
	}
	pub fn cols(&self) -> u16 { self.max_col }
	pub fn rows(&self) -> u16 { self.max_row }
	pub fn room(&self, col: u16, row: u16) -> Option<&Room> {
		self.rooms.get(&(col, row))
	}
	pub fn room_mut(&mut self, col: u16, row: u16) -> &mut Room {
		if self.rooms.get(&(col, row)).is_none() {
			self.rooms.insert((col, row), Room::default());
		}
		self.rooms.get_mut(&(col, row)).expect("room")
	}
	pub fn add_fills(&mut self, fills: Vec<Fill>) {
		for fill in fills {
			for row in fill.frame.row_range() {
				for col in fill.frame.col_range() {
					if let Some((col, row)) = self.in_range_col_row(col, row) {
						let z = fill.frame.z;
						match fill.glyph {
							Glyph::Rune(char, color) => {
								let rune = Rune(char, solar_dark::color_by_index(color));
								self.room_mut(col, row).add_rune(z, rune);
							}
							Glyph::Tile(color) => {
								let tile = Tile(solar_dark::color_by_index(color));
								self.room_mut(col, row).add_tile(z, tile);
							}
						}
					}
				}
			}
		}
	}
	pub fn print_to(&self, console: &mut Console) {
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

impl Screen {
	pub fn new((cols, rows): (u16, u16)) -> Self {
		Self { max_col: cols, max_row: rows, ..Screen::default() }
	}
}

#[derive(Default)]
pub struct Room {
	z_tiles: BTreeMap<i16, Tile>,
	z_runes: BTreeMap<i16, Rune>,
}

impl Room {
	pub fn add_tile(&mut self, z: i16, tile: Tile) {
		self.z_tiles.insert(-z, tile);
	}
	pub fn add_rune(&mut self, z: i16, rune: Rune) {
		self.z_runes.insert(-z, rune);
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
			Rune(' ', Color::Magenta)
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

