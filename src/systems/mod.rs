use bevy::prelude::{Commands, Component, default, Entity, Query, Res, Transform, With};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::components::{View, ViewModel};
use crate::components::layout::{Layout, LayoutFn};
use crate::components::render::Render;
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

pub struct SampleApp;

impl ViewModel<()> for SampleApp {
	fn to_layouts(&self) -> Vec<Layout> {
		let louter = Layout { layout: sample_layout() };
		vec![louter]
	}
}

pub fn add_root_view(mut commands: Commands) {
	let view = View {
		view_model: Box::new(SampleApp {}),
	};
	commands.spawn((RootView, view));
}

#[derive(Component)]
pub struct RootView;

pub fn spawn_view_layouts<ViewMsg: 'static>(query: Query<&View<ViewMsg>, With<RootView>>, mut commands: Commands) {
	for view in query.iter() {
		for louter in view.view_model.to_layouts() {
			commands.spawn((ViewLayout, louter));
		}
	}
}

pub fn despawn_view_layouts(query: Query<Entity, With<ViewLayout>>, mut commands: Commands) {
	for entity in query.iter() {
		commands.entity(entity).despawn();
	}
}

#[derive(Component)]
pub struct ViewLayout;


fn sample_layout() -> Box<LayoutFn> {
	let layout = Box::new(|volume: Volume| {
		let volume = volume.inset(Inset::DoubleCols(1));
		vec![Render { volume, render: tools::render::sample_render() }]
	});
	layout
}
