use bevy::prelude::{ColorMaterial, Component, Handle, Mesh, Resource};

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
pub struct AppAssets {
	pub color_materials: Vec<Handle<ColorMaterial>>,
	pub meshes: Vec<Handle<Mesh>>,
}

#[derive(Component)]
pub struct FillMesh;

#[derive(Component, Copy, Clone)]
pub struct Fill {
	pub glyph: Glyph,
	pub volume: Volume,
}

impl Fill {
	pub fn left(&self) -> f32 {
		self.volume.left as f32
	}
	pub fn top(&self) -> f32 {
		self.volume.top as f32
	}
	pub fn width(&self) -> f32 {
		self.volume.width() as f32
	}
	pub fn height(&self) -> f32 {
		self.volume.height() as f32
	}
	pub fn near(&self) -> f32 {
		self.volume.near as f32
	}
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

impl Volume {
	pub fn width(&self) -> i16 {
		self.right - self.left
	}
	pub fn height(&self) -> i16 {
		self.bottom - self.top
	}
}

#[derive(Copy, Clone)]
pub enum Glyph {
	Solid(usize)
}
