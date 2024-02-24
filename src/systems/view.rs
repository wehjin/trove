use bevy::prelude::{Commands, Component, Entity, Query, With};

use crate::components::layout::RootLayout;
use crate::components::view::View;

#[derive(Component)]
pub struct RootView;

pub fn spawn_root_view_layouts<ViewMsg: 'static>(query: Query<&View<ViewMsg>, With<RootView>>, mut commands: Commands) {
	let view = query.single();
	for louter in view.model.to_layouts() {
		commands.spawn((ViewLayout, louter, RootLayout));
	}
}

pub fn despawn_view_layouts(query: Query<Entity, With<ViewLayout>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

#[derive(Component)]
pub struct ViewLayout;
