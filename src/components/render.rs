use bevy::prelude::Component;

use crate::tools::RenderFn;

#[derive(Component)]
pub struct RendererMadeFill;

#[derive(Component)]
pub struct Renderer {
	pub render: Box<RenderFn>,
}
