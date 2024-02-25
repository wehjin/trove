use bevy::prelude::{Commands, Entity, Query, Res, With};

use crate::components::layout::{Louter, LouterMadeRenderer, RootLouter};
use crate::components::render::Renderer;
use crate::tools::console::Console;
use crate::tools::volume::Volume;

pub fn despawn_layout_renderers(query: Query<Entity, With<LouterMadeRenderer>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn spawn_root_layout_renderers(query: Query<&Louter, With<RootLouter>>, console: Res<Console>, mut commands: Commands) {
	let (cols, rows) = console.width_height();
	// TODO Fix this for non-root layouts. It is correct only for root layouts.
	let volume = Volume::from_cols_rows_near(cols, rows, 1);
	let louter = query.single();
	let renders = louter.layout.run_layout(volume);
	for render in renders {
		commands.spawn((LouterMadeRenderer, Renderer { render }));
	}
}
