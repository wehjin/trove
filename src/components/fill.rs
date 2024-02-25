use bevy::prelude::Component;

use crate::tools::fill::Glyph;
use crate::tools::volume::Volume;

#[derive(Component)]
pub struct FillMadeMesh;

#[derive(Component, Clone)]
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
