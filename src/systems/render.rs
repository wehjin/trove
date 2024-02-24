use bevy::prelude::{Commands, Entity, Query, With};

use crate::components::fill::Fill;
use crate::components::render::{Render, RenderFill};

pub fn despawn_renderer_fills(query: Query<Entity, With<RenderFill>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn spawn_renderer_fills(query: Query<&Render>, mut commands: Commands) {
	for renderer in query.iter() {
		let fills: Vec<Fill> = (renderer.render)(renderer.volume);
		for fill in fills {
			commands.spawn((RenderFill, fill));
		}
	}
}
