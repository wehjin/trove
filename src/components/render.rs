use bevy::prelude::Component;

use crate::components::fill::Fill;
use crate::tools::volume::Volume;

#[derive(Component)]
pub struct VolumeFillComponent {
	pub volume: Volume,
	pub fill: Box<FillFn>,
}

pub type FillFn = dyn Fn(Volume) -> Vec<Fill> + Send + Sync;

#[derive(Component)]
pub struct EphemeralFill;
