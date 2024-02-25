use bevy::prelude::Component;

use crate::tools::ViewModel;

#[derive(Component)]
pub struct ViewMadeLayout;

#[derive(Component)]
pub struct View<Msg> {
	pub model: Box<dyn ViewModel<Msg> + Send + Sync>,
}

#[derive(Component)]
pub struct RootView;

