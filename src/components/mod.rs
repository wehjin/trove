use bevy::prelude::Component;

use crate::components::layout::Louter;

pub mod console;
pub mod fill;
pub mod layout;
pub mod render;
pub mod setup;

pub trait View<ViewMsg> {
	fn to_louters(&self) -> Vec<Louter>;
}

#[derive(Component)]
pub struct Viewer<Msg> {
	pub view: Box<dyn View<Msg> + Send + Sync>,
}

