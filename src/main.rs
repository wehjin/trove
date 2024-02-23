use std::error::Error;

use bevy::prelude::{App, Commands, Component, IntoSystemConfigs, Query, Res, Startup, Update, With};
use crossterm::event::read;
use crossterm::style::Color;

use console::Console;

fn main() -> Result<(), Box<dyn Error>> {
	Console::start()?;
	App::new()
		.add_systems(Startup, add_console)
		.add_systems(Startup, add_panels.after(add_console))
		.add_systems(Update, hello_world)
		.add_systems(Update, greet_panels.after(hello_world))
		.add_systems(Update, await_human.after(greet_panels))
		.run();
	Console::stop()?;
	Ok(())
}

fn add_console(mut commands: Commands) {
	let console = Console::start().expect("start");
	commands.insert_resource(console);
}

fn await_human() {
	read().expect("read");
}

fn hello_world(console: Res<Console>) {
	console.move_print(5, 0, "hello world!");
}

fn add_panels(mut commands: Commands) {
	commands.spawn((Panel, Position { left: 0, top: 15, right: 20, bottom: 20, near: 0, far: 0 }));
	commands.spawn((Panel, Position { left: 35, top: 5, right: 40, bottom: 40, near: 0, far: 0 }));
}

fn greet_panels(console: Res<Console>, query: Query<&Position, With<Panel>>) {
	for pos in &query {
		console.color(pos, Color::Green);
	}
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
