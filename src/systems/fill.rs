use bevy::prelude::{Commands, default, Entity, Query, Res, Transform, With};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::components::fill::{Fill, FillMadeMesh};
use crate::components::setup::AppAssets;
use crate::tools::console::Console;
use crate::tools::fill::Glyph;

pub fn despawn_fill_meshes(query: Query<Entity, With<FillMadeMesh>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn spawn_fill_meshes(query: Query<&Fill>, app_assets: Res<AppAssets>, console: Res<Console>, mut commands: Commands) {
	let (_cols, rows) = console.width_height();
	for fill in query.iter() {
		let (color_index, mesh_index) = match &fill.glyph {
			Glyph::Solid(color_index) => (*color_index, 1),
			Glyph::Text(color_index) => (*color_index, 0),
		};
		let transform = {
			let center = Transform::from_xyz(0.5, -0.5, 0.);
			let scale = Transform::from_scale((fill.width(), fill.height(), 1.).into());
			let shift = Transform::from_xyz(fill.left(), rows as f32 - fill.top(), fill.near());
			let together = shift.compute_matrix().mul_mat4(&scale.compute_matrix()).mul_mat4(&center.compute_matrix());
			Transform::from_matrix(together)
		};
		let material = app_assets.color_materials[color_index].clone();
		commands.spawn((FillMadeMesh, MaterialMesh2dBundle {
			mesh: Mesh2dHandle(app_assets.meshes[mesh_index].clone()),
			material,
			transform,
			..default()
		}));
	}
}
