use bevy::prelude::{Commands, Component, default, Entity, Query, Res, Transform, With};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::components::{View, Viewer};
use crate::components::layout::{LayoutFn, Louter};
use crate::components::render::Renderer;
use crate::components::setup::AppAssets;
use crate::tools;
use crate::tools::console::Console;
use crate::tools::inset::Inset;
use crate::tools::volume::Volume;

pub mod console;
pub mod fill;
pub mod layout;
pub mod render;
pub mod setup;

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

pub struct SampleView;

impl View<()> for SampleView {
	fn to_louters(&self) -> Vec<Louter> {
		let louter = Louter { layout: sample_layout() };
		vec![louter]
	}
}

#[derive(Component)]
pub struct RootViewer;

pub fn add_root_viewer(mut commands: Commands) {
	let viewer = Viewer {
		view: Box::new(SampleView {}),
	};
	commands.spawn((RootViewer, viewer));
}

pub fn spawn_viewer_louters<ViewMsg: 'static>(query: Query<&Viewer<ViewMsg>, With<RootViewer>>, mut commands: Commands) {
	for viewer in query.iter() {
		for louter in viewer.view.to_louters() {
			commands.spawn((ViewerLouter, louter));
		}
	}
}

pub fn despawn_viewer_louters(query: Query<Entity, With<ViewerLouter>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

#[derive(Component)]
pub struct ViewerLouter;


fn sample_layout() -> Box<LayoutFn> {
	let layout = Box::new(|volume: Volume| {
		let volume = volume.inset(Inset::DoubleCols(1));
		vec![Renderer { volume, render: tools::render::sample_render() }]
	});
	layout
}
