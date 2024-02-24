use bevy::prelude::Component;

use crate::components::fill::Fill;
use crate::tools::volume::Volume;

#[derive(Component)]
pub struct RenderFill;

#[derive(Component)]
pub struct Render {
	pub volume: Volume,
	pub render: Box<RenderFn>,
}

pub type RenderFn = dyn Fn(Volume) -> Vec<Fill> + Send + Sync;
