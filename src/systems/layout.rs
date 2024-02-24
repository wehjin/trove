use bevy::prelude::{Commands, Entity, Query, Res, With};

use crate::components::layout::{Louter, LouterRenderer};
use crate::components::render::Renderer;
use crate::tools::console::Console;
use crate::tools::volume::Volume;

pub fn spawn_louter_renderers(query: Query<&Louter>, console: Res<Console>, mut commands: Commands) {
	let (cols, rows) = console.width_height();
	let volume = Volume::from_cols_rows_near(cols, rows, 1);
	for louter in query.iter() {
		let renderers: Vec<Renderer> = (louter.layout)(volume);
		for renderer in renderers {
			commands.spawn((LouterRenderer, renderer));
		}
	}
}

pub fn despawn_louter_renderers(query: Query<Entity, With<LouterRenderer>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}
