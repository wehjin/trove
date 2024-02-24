use bevy::prelude::{ColorMaterial, Component, Handle, Mesh, Resource};

pub type RenderFn = dyn Fn(Volume) -> Vec<Fill> + Send + Sync;


#[derive(Component)]
pub struct RendererFill;

#[derive(Component)]
pub struct RootRenderer;

#[derive(Component)]
pub struct Renderer {
	pub render: Box<RenderFn>,
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

pub enum Inset {
	DoubleCols(u8),
}

impl Inset {
	pub fn to_top_right_bottom_left(&self) -> (u8, u8, u8, u8) {
		match self {
			Inset::DoubleCols(size) => (*size, 2 * size, *size, 2 * size),
		}
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
	pub fn from_cols_rows_near(cols: u16, rows: u16, near: i16) -> Self {
		Self { left: 0, top: 0, far: 0, right: cols as i16, bottom: rows as i16, near }
	}
	pub fn inset(mut self, inset: Inset) -> Self {
		let (t, r, b, l) = inset.to_top_right_bottom_left();
		self.top += t as i16;
		self.right -= r as i16;
		self.bottom -= b as i16;
		self.left += l as i16;
		self
	}

	pub fn split_from_top(mut self, rows: u16) -> (Volume, Volume) {
		let split = self.top + rows as i16;
		let bottom = Volume { top: split, ..self.clone() };
		self.bottom = split;
		(self, bottom)
	}
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

#[derive(Resource)]
pub struct AppAssets {
	pub color_materials: Vec<Handle<ColorMaterial>>,
	pub meshes: Vec<Handle<Mesh>>,
}

#[derive(Component)]
pub struct OrthoCam;

pub mod console;
