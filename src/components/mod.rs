use bevy::prelude::Component;

use crate::components::layout::Layout;

pub mod console;
pub mod fill;
pub mod layout;
pub mod render;
pub mod setup;

pub trait ViewModel<ViewMsg> {
	fn to_layouts(&self) -> Vec<Layout>;
}

#[derive(Component)]
pub struct View<Msg> {
	pub view_model: Box<dyn ViewModel<Msg> + Send + Sync>,
}

