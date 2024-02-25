use bevy::prelude::{Commands, Entity, Query, Res, With};

use crate::components::layout::{VolumeRendererComponent, VolumeRendererSeatedRenderer, RootVolumeRenderer};
use crate::tools::console::Console;
use crate::tools::volume::Volume;

pub fn despawn_layout_renderings(query: Query<Entity, With<VolumeRendererSeatedRenderer>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn spawn_root_layout_renderings(query: Query<&VolumeRendererComponent, With<RootVolumeRenderer>>, console: Res<Console>, mut commands: Commands) {
	let (cols, rows) = console.width_height();
	// TODO Fix this for non-root layouts. It is correct only for root layouts.
	let volume = Volume::from_cols_rows_near(cols, rows, 1);
	for layout in query.iter() {
		let renders = (layout.render_volume)(volume);
		for render in renders {
			commands.spawn((VolumeRendererSeatedRenderer, render));
		}
	}
}
