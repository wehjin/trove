use bevy::prelude::{ColorMaterial, Component, Handle, Resource};

#[derive(Component)]
pub struct AppCamera;

#[derive(Component)]
pub struct Panel;

#[derive(Component, Copy, Clone, Eq, PartialEq)]
pub struct Position {
	pub left: u16,
	pub top: u16,
	pub right: u16,
	pub bottom: u16,
	pub near: u16,
	pub far: u16,
}

#[derive(Resource)]
pub struct Palette {
	pub color_materials: Vec<Handle<ColorMaterial>>,
}

#[derive(Component, Copy, Clone)]
pub struct Fill {
	pub glyph: Glyph,
	pub volume: Volume,
}

#[derive(Copy, Clone)]
pub struct Volume {
	pub left: i16,
	pub top: i16,
	pub far: i16,
	pub right: i16,
	pub bottom: i16,
	pub near: i16,
}

#[derive(Copy, Clone)]
pub enum Glyph {
	Solid
}
