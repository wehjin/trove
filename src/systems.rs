use bevy::asset::Assets;
use bevy::prelude::{Camera2dBundle, Circle, ColorMaterial, Commands, default, Mesh, Query, Res, ResMut, Transform, With};
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crossterm::event::read;
use crossterm::style::Color;

use crate::components::{AppCamera, Palette, Panel, Position};
use crate::console::Console;

pub fn add_palette(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
	let colors = {
		use bevy::prelude::*;
		[Color::SEA_GREEN, Color::ALICE_BLUE]
	};
	let mut handles = Vec::new();
	for color in colors {
		let material = materials.add(color);
		handles.push(material);
	}
	let palette = Palette {
		color_materials: handles,
	};
	commands.insert_resource(palette);
}

pub fn setup_camera(mut commands: Commands, console: Res<Console>) {
	let (width, height) = console.width_height();
	let mut camera = Camera2dBundle { ..default() };
	camera.projection.scaling_mode = ScalingMode::Fixed { width: width as f32, height: height as f32 };
	camera.projection.viewport_origin = (0., 0.).into();
	commands.spawn((AppCamera, camera));
}

pub fn add_circles(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	palette: Res<Palette>,
	console: Res<Console>,
) {
	let shape = Mesh2dHandle(meshes.add(Circle { radius: 0.45 }));
	let (width, height) = console.width_height();
	for row in 0..height {
		let y = row as f32 + 0.5;
		for col in 0..width {
			let x = col as f32 + 0.5;
			if (col % 10) != 0 {
				commands.spawn(MaterialMesh2dBundle {
					mesh: shape.clone(),
					material: palette.color_materials[row as usize % 2].clone(),
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

pub fn await_human() {
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
		console.color(pos, Color::Green);
	}
}

pub fn flush_console(mut console: ResMut<Console>) {
	console.flush();
}

