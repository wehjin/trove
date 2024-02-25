use bevy::prelude::{Commands, Entity, Query, With};

use crate::components::fill::Fill;
use crate::components::render::{RendererMadeFill, Renderer};

pub fn despawn_renderer_fills(query: Query<Entity, With<RendererMadeFill>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn spawn_renderer_fills(query: Query<&Renderer>, mut commands: Commands) {
	for renderer in query.iter() {
		let fills: Vec<Fill> = (renderer.render)();
		for fill in fills {
			commands.spawn((RendererMadeFill, fill));
		}
	}
}
