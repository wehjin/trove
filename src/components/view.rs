use bevy::prelude::Component;

use crate::components::layout::VolumeRenderer;

pub trait ViewModel<ViewMsg> {
	fn to_volume_renderers(&self) -> Vec<Box<VolumeRenderer>>;
}

#[derive(Component)]
pub struct View<Msg> {
	pub model: Box<dyn ViewModel<Msg> + Send + Sync>,
}

#[derive(Component)]
pub struct RootView;

#[derive(Component)]
pub struct TempVolumeRenderer;

