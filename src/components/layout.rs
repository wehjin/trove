use bevy::prelude::Component;

use crate::components::render::Render;
use crate::tools::volume::Volume;

#[derive(Component)]
pub struct Layout {
	pub layout: Box<LayoutFn>,
}

pub type LayoutFn = dyn Fn(Volume) -> Vec<Render> + Send + Sync;

#[derive(Component)]
pub struct LayoutRender;
