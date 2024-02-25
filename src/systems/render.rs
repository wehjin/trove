use bevy::prelude::{Commands, Entity, Query, With};

use crate::components::render::{Renderer, RendererMadeFill};

pub fn despawn_renderer_fills(query: Query<Entity, With<RendererMadeFill>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn spawn_renderer_fills(query: Query<&Renderer>, mut commands: Commands) {
	for renderer in query.iter() {
		for fill in renderer.render.run_render() {
			commands.spawn((RendererMadeFill, fill));
		}
	}
}
