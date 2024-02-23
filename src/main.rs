use std::error::Error;

use bevy::DefaultPlugins;
use bevy::prelude::{App, Assets, Camera2dBundle, Circle, ColorMaterial, Commands, Component, default, IntoSystemConfigs, Mesh, Query, Res, ResMut, Startup, Transform, Update, With};
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crossterm::event::read;
use crossterm::style::Color;

use console::Console;

fn main() -> Result<(), Box<dyn Error>> {
	Console::start()?;
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, add_console)
		.add_systems(Startup, add_panels.after(add_console))
		.add_systems(Startup, setup_camera.after(add_console))
		.add_systems(Update, flush_console)
		.add_systems(Update, hello_world.before(flush_console))
		.add_systems(Update, greet_panels.after(hello_world).before(flush_console))
		.run();
	Console::stop()?;
	Ok(())
}

#[derive(Component)]
pub struct AppCamera;

fn setup_camera(
	mut commands: Commands,
	console: Res<Console>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let (width, height) = console.width_height();

	let mut camera = Camera2dBundle { ..default() };
	camera.projection.scaling_mode = ScalingMode::Fixed { width: width as f32, height: height as f32 };
	camera.projection.viewport_origin = (0., 0.).into();
	commands.spawn((AppCamera, camera));

	let shape = Mesh2dHandle(meshes.add(Circle { radius: 0.45 }));
	let colors = {
		use bevy::prelude::*;
		[Color::SEA_GREEN, Color::AQUAMARINE]
	};
	let color_materials = colors.into_iter()
		.map(|color| materials.add(color))
		.collect::<Vec<_>>();

	for row in 0..height {
		let y = row as f32 + 0.5;
		for col in 0..width {
			let x = col as f32 + 0.5;
			if (col % 10) != 0 {
				commands.spawn(MaterialMesh2dBundle {
					mesh: shape.clone(),
					material: color_materials[row as usize % 2].clone(),
					transform: Transform::from_xyz(x, y, 0.0),
					..default()
				});
			}
		}
	}
}

fn add_console(mut commands: Commands) {
	let console = Console::start().expect("start");
	commands.insert_resource(console);
}

fn await_human() {
	read().expect("read");
}

fn add_panels(mut commands: Commands) {
	commands.spawn((Panel, Position { left: 0, top: 15, right: 20, bottom: 20, near: 0, far: 0 }));
	commands.spawn((Panel, Position { left: 35, top: 5, right: 40, bottom: 40, near: 0, far: 0 }));
}

fn hello_world(console: Res<Console>) {
	console.move_print(0, 0, "hello world!");
}

fn greet_panels(console: Res<Console>, query: Query<&Position, With<Panel>>) {
	for pos in &query {
		console.color(pos, Color::Green);
	}
}

fn flush_console(mut console: ResMut<Console>) {
	console.flush();
}

#[derive(Component)]
struct Panel;

#[derive(Component, Copy, Clone, Eq, PartialEq)]
pub struct Position {
	left: u16,
	top: u16,
	right: u16,
	bottom: u16,
	near: u16,
	far: u16,
}

pub mod console;
