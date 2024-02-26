use bevy::prelude::Component;

use crate::tools::ViewUpdating;

#[derive(Component)]
pub struct ViewComponent {
	pub model: Box<dyn ViewUpdating + Send + Sync>,
}

#[derive(Component)]
pub struct RootViewMarker;

