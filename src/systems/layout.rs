use bevy::prelude::{Commands, Entity, Query, Res, With};

use crate::components::layout::{Layout, LayoutRender, RootLayout};
use crate::tools::console::Console;
use crate::tools::volume::Volume;

pub fn spawn_root_layout_renders(query: Query<&Layout, With<RootLayout>>, console: Res<Console>, mut commands: Commands) {
	let (cols, rows) = console.width_height();
	// TODO Fix this for non-root layouts. It is correct only for root layouts.
	let volume = Volume::from_cols_rows_near(cols, rows, 1);
	for layout in query.iter() {
		let renders = (layout.layout)(volume);
		for render in renders {
			commands.spawn((LayoutRender, render));
		}
	}
}

pub fn despawn_layout_renders(query: Query<Entity, With<LayoutRender>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}
