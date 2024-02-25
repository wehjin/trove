use bevy::prelude::Component;

use crate::tools::Render;

#[derive(Component)]
pub struct RendererMadeFill;

#[derive(Component)]
pub struct Renderer {
	pub render: Box<dyn Render + Send + Sync>,
}
