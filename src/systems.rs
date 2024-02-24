use bevy::asset::Assets;
use bevy::prelude::{Camera2dBundle, Circle, ColorMaterial, Commands, default, Entity, Mesh, Query, Rectangle, Res, ResMut, Transform, With};
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crossterm::event::read;
use crossterm::style;

use crate::components::{AppAssets, AppCamera, Fill, FillMesh, Glyph, Panel, Position, Volume};
use crate::console::Console;
use crate::resources::Palette16;

pub fn setup_camera(mut commands: Commands, console: Res<Console>) {
	let (width, height) = console.width_height();
	let mut camera = Camera2dBundle { ..default() };
	camera.projection.scaling_mode = ScalingMode::Fixed { width: width as f32, height: height as f32 };
	camera.projection.viewport_origin = (0., 0.).into();
	commands.spawn((AppCamera, camera));
}

pub fn add_app_assets(
	palette: Res<Palette16>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut commands: Commands,
) {
	let app_assets = AppAssets {
		color_materials: palette.to_colors()
			.into_iter()
			.map(|c| materials.add(c))
			.collect::<Vec<_>>(),
		meshes: vec![
			meshes.add(Circle { radius: 0.45 }),
			meshes.add(Rectangle::new(1.0, 1.0)),
		],
	};
	commands.insert_resource(app_assets);
}

pub fn add_fills(console: Res<Console>, mut commands: Commands) {
	let (cols, rows) = console.width_height();
	let fills = [
		Fill {
			glyph: Glyph::Solid(0),
			volume: Volume { left: 1, top: 2, far: 0, right: cols as i16 - 1, bottom: rows as i16 - 1, near: 1 },
		},
		Fill {
			glyph: Glyph::Solid(1),
			volume: Volume { left: 1, top: 1, far: 0, right: cols as i16 - 1, bottom: 2, near: 1 },
		},
	];
	commands.spawn(fills[0]);
	commands.spawn(fills[1]);
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
		let color_index = match fill.glyph {
			Glyph::Solid(color_index) => color_index,
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
			mesh: Mesh2dHandle(app_assets.meshes[1].clone()),
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

pub fn add_console(mut commands: Commands) {
	let console = Console::start().expect("start");
	commands.insert_resource(console);
}

pub fn _await_human() {
	read().expect("read");
}

pub fn add_panels(mut commands: Commands) {
	commands.spawn((Panel, Position { left: 0, top: 15, right: 20, bottom: 20, near: 0, far: 0 }));
	commands.spawn((Panel, Position { left: 35, top: 5, right: 40, bottom: 40, near: 0, far: 0 }));
}

pub fn hello_world(console: Res<Console>) {
	console.move_print(0, 0, "hello world!");
}

pub fn greet_panels(console: Res<Console>, query: Query<&Position, With<Panel>>) {
	for pos in &query {
		console.color(pos, style::Color::Green);
	}
}

pub fn flush_console(mut console: ResMut<Console>) {
	console.flush();
}

