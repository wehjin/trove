use bevy::prelude::{Commands, Entity, Query, With};

use crate::components::layout::{RootVolumeRenderer, VolumeRendererComponent};
use crate::components::view::{RootView, TempVolumeRenderer, View};

pub fn despawn_volume_renderers(query: Query<Entity, With<TempVolumeRenderer>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn spawn_root_view_volume_renderers<ViewMsg: 'static>(query: Query<&View<ViewMsg>, With<RootView>>, mut commands: Commands) {
	let view = query.single();
	for renderer in view.model.to_volume_renderers() {
		let layout = VolumeRendererComponent { render_volume: renderer };
		commands.spawn((TempVolumeRenderer, layout, RootVolumeRenderer));
	}
}
