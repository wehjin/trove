use bevy::prelude::Component;
use crate::components::layout::Layout;

pub trait ViewModel<ViewMsg> {
	fn to_layouts(&self) -> Vec<Layout>;
}

#[derive(Component)]
pub struct View<Msg> {
	pub model: Box<dyn ViewModel<Msg> + Send + Sync>,
}

