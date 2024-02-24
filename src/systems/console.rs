use bevy::prelude::{Commands, Query, Res, ResMut, With};
use crossterm::style;
use crossterm::event::read;
use crate::components::console::{Panel, Position};
use crate::console::Console;

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

