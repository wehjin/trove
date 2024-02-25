use bevy::prelude::Component;

use crate::components::render::VolumeFillComponent;
use crate::tools::volume::Volume;

#[derive(Component)]
pub struct VolumeRendererComponent {
	pub render_volume: Box<VolumeRenderer>,
}

pub type VolumeRenderer = dyn Fn(Volume) -> Vec<VolumeFillComponent> + Send + Sync;

#[derive(Component)]
pub struct VolumeRendererSeatedRenderer;

#[derive(Component)]
pub struct RootVolumeRenderer;
