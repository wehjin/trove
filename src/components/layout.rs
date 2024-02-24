use bevy::prelude::Component;

use crate::components::render::Renderer;
use crate::tools::volume::Volume;

#[derive(Component)]
pub struct Layout {
	pub layout: Box<LayoutFn>,
}

pub type LayoutFn = dyn Fn(Volume) -> Vec<Renderer> + Send + Sync;

#[derive(Component)]
pub struct LayoutRender;

#[derive(Component)]
pub struct RootLayout;
