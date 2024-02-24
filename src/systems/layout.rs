use bevy::prelude::{Commands, Entity, Query, Res, With};
use crate::components::layout::{Louter, LouterRenderer, RootLouter};
use crate::components::render::Renderer;
use crate::tools;
use crate::tools::console::Console;
use crate::tools::{Inset, Volume};

pub fn spawn_louter_renderers(query: Query<&Louter, With<RootLouter>>, console: Res<Console>, mut commands: Commands) {
	let (cols, rows) = console.width_height();
	let volume = Volume::from_cols_rows_near(cols, rows, 1);
	for louter in query.iter() {
		let renderers: Vec<Renderer> = (louter.layout)(volume);
		for renderer in renderers {
			commands.spawn((LouterRenderer, renderer));
		}
	}
}

pub fn despawn_louter_renderers(query: Query<Entity, With<LouterRenderer>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

pub fn add_root_louter(mut commands: Commands) {
	let layout = Box::new(|volume: Volume| {
		let volume = volume.inset(Inset::DoubleCols(1));
		vec![Renderer { volume, render: tools::render::sample_render() }]
	});
	commands.spawn((RootLouter, Louter { layout }));
}
