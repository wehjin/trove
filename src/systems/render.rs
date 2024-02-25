use bevy::prelude::{Commands, Entity, Query, With};

use crate::components::fill::Fill;
use crate::components::render::{VolumeFillComponent, EphemeralFill};

pub fn despawn_rendering_fills(query: Query<Entity, With<EphemeralFill>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn spawn_rendering_fills(query: Query<&VolumeFillComponent>, mut commands: Commands) {
	for renderer in query.iter() {
		let fills: Vec<Fill> = (renderer.fill)(renderer.volume);
		for fill in fills {
			commands.spawn((EphemeralFill, fill));
		}
	}
}
