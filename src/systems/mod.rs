use bevy::prelude::{Commands, default, Entity, Query, Res, Transform, With};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::components::{AppAssets, Fill, FillMesh, Glyph, Inset, Renderer, RendererFill, RootRenderer, Volume};
use crate::resources::solar_dark;
use crate::tools::console::Console;

pub fn add_root_renderer(mut commands: Commands) {
	let renderer = Renderer {
		render: Box::new(|volume| {
			let (head_volume, body_volume) = volume.split_from_top(1);
			let mut vec = vec![
				Fill { glyph: Glyph::Solid(solar_dark::BASE03), volume: body_volume },
				Fill { glyph: Glyph::Solid(solar_dark::BASE02), volume: head_volume },
			];
			vec.extend(string_to_fills(
				"hello world!",
				head_volume.clone().move_closer(1),
				solar_dark::BASE1,
			));
			vec.extend(string_to_fills(
				"rack the dubs, chad",
				body_volume.clone().move_closer(1),
				solar_dark::BASE0,
			));
			vec
		}),
	};
	commands.spawn((RootRenderer, renderer));
}

fn string_to_fills(string: &str, string_volume: Volume, color_index: usize) -> Vec<Fill> {
	let mut fill_volume = string_volume.with_width_from_left(1).with_height_from_top(1);
	let mut vec = Vec::new();
	for i in 0..string.chars().count() {
		if !&string[i..i + 1].trim().is_empty() {
			let fill = Fill { glyph: Glyph::Text(color_index), volume: fill_volume.clone() };
			vec.push(fill);
		}
		fill_volume = fill_volume.move_right(1);
	}
	vec
}

pub fn despawn_renderer_fills(query: Query<Entity, With<RendererFill>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn spawn_renderer_fills(query: Query<&Renderer, With<RootRenderer>>, console: Res<Console>, mut commands: Commands) {
	let (cols, rows) = console.width_height();
	for renderer in query.iter() {
		let volume = Volume::from_cols_rows_near(cols, rows, 1).inset(Inset::DoubleCols(1));
		let fills: Vec<Fill> = (renderer.render)(volume);
		for fill in fills {
			commands.spawn((RendererFill, fill));
		}
	}
}

pub fn despawn_fill_meshes(query: Query<Entity, With<FillMesh>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn spawn_fill_meshes(
	query: Query<&Fill>,
	app_assets: Res<AppAssets>,
	console: Res<Console>,
	mut commands: Commands,
) {
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
		commands.spawn((FillMesh, MaterialMesh2dBundle {
			mesh: Mesh2dHandle(app_assets.meshes[mesh_index].clone()),
			material,
			transform,
			..default()
		}));
	}
}

pub fn add_circles(mut commands: Commands, palette_mesh: Res<AppAssets>, console: Res<Console>) {
	let (width, height) = console.width_height();
	for row in 0..height {
		let y = row as f32 + 0.5;
		for col in 0..width {
			let x = col as f32 + 0.5;
			if (col % 10) != 9 {
				commands.spawn(MaterialMesh2dBundle {
					mesh: Mesh2dHandle(palette_mesh.meshes[0].clone()),
					material: palette_mesh.color_materials[if ((row + col) as usize % 2) == 0 { 12 } else { 14 }].clone(),
					transform: Transform::from_xyz(x, y, 0.0),
					..default()
				});
			}
		}
	}
}

pub mod console;
pub mod setup;