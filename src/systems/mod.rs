use bevy::prelude::{Commands, default, Res, Transform};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::components::view::RootView;

use crate::components::layout::VolumeRenderer;
use crate::components::render::VolumeFillComponent;
use crate::components::setup::AppAssets;
use crate::components::view::{View, ViewModel};
use crate::tools;
use crate::tools::console::Console;
use crate::tools::inset::Inset;
use crate::tools::volume::Volume;

pub mod console;
pub mod fill;
pub mod layout;
pub mod render;
pub mod setup;
pub mod view;

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
	fn to_volume_renderers(&self) -> Vec<Box<VolumeRenderer>> {
		vec![Box::new(sample_layout)]
	}
}

fn sample_layout(volume: Volume) -> Vec<VolumeFillComponent> {
	let volume = volume.inset(Inset::DoubleCols(1));
	vec![VolumeFillComponent { volume, fill: tools::render::sample_fill_builder() }]
}

pub fn add_root_view(mut commands: Commands) {
	let view = View { model: Box::new(SampleApp {}) };
	commands.spawn((RootView, view));
}
