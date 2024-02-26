use bevy::prelude::Component;

use crate::tools::ShapePaint;

#[derive(Component)]
pub struct ViewComponent {
	pub model: Box<dyn ShapePaint + Send + Sync>,
}

#[derive(Component)]
pub struct RootViewMarker;

