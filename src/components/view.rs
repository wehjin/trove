use bevy::prelude::Component;

use crate::tools::ViewUpdating;

#[derive(Component)]
pub struct ModelInputs<Msg> {
	pub model: Box<dyn ViewUpdating<Msg=Msg> + Send + Sync>,
	pub msg_queue: Vec<Msg>,
}

#[derive(Component)]
pub struct RootViewMarker;

