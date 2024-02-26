use bevy::prelude::{Bundle, Changed, Commands, Component, default, Entity, Query, Res, ResMut, Transform};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::RootViewBuilder;
use crate::components::fill::Fill;
use crate::components::setup::AppAssets;
use crate::components::view::{RootViewMarker, ViewComponent};
use crate::tools::{Painter, Shaper, ShapeResult, ViewBuilding};
use crate::tools::console::Console;
use crate::tools::fill::Glyph;
use crate::tools::sample::SampleAppSettings;
use crate::tools::zrect::ZRect;

pub mod console;
pub mod setup;

#[derive(Component, Default)]
pub struct ShaperInputs {
	shaper: Option<Box<dyn Shaper + Send + Sync>>,
	edge_zrect: Option<ZRect>,
}

#[derive(Component, Default)]
pub struct PainterInputs {
	painters: Vec<Box<dyn Painter + Send + Sync>>,
}

#[derive(Component, Default)]
pub struct MeshInputs {
	fills: Vec<Fill>,
	max_row: u16,
}

#[derive(Component, Default)]
pub struct MeshOutputs {
	mesh_ids: Vec<Entity>,
}

#[derive(Bundle)]
struct ViewBundle {
	view: ViewComponent,
	shaper_inputs: ShaperInputs,
	painter_inputs: PainterInputs,
	mesh_inputs: MeshInputs,
	mesh_outputs: MeshOutputs,
}

pub struct ViewEffects<'a, 'w, 's> {
	pub commands: &'a mut Commands<'w, 's>,
	pub shaper: Option<Box<dyn Shaper + Send + Sync>>,
}

impl<'a, 'w, 's> ViewEffects<'a, 'w, 's> {
	pub fn set_shaper(&mut self, shaper: impl Shaper + Send + Sync + 'static) {
		self.shaper = Some(Box::new(shaper));
	}
}

pub fn add_root_view(console: Res<Console>, mut builder: ResMut<RootViewBuilder<SampleAppSettings>>, mut commands: Commands) {
	let (cols, rows) = console.width_height();
	let builder = builder.value.take().expect("no ViewModelBuilder");
	let mut effects = ViewEffects { commands: &mut commands, shaper: None };
	let model = builder.init_view(&mut effects);
	let shaper_inputs = ShaperInputs {
		shaper: effects.shaper,
		edge_zrect: Some(ZRect::from_cols_rows_z(cols, rows, 1)),
	};
	let painter_inputs = PainterInputs { painters: Vec::new() };
	let mesh_inputs = MeshInputs { fills: Vec::new(), max_row: rows };
	let mesh_outputs = MeshOutputs::default();
	let view = ViewComponent { model: Box::new(model) };
	commands.spawn((RootViewMarker, ViewBundle { view, shaper_inputs, painter_inputs, mesh_inputs, mesh_outputs }));
}

pub fn apply_shapers_update_painters(mut query: Query<(&mut ShaperInputs, &mut PainterInputs), Changed<ShaperInputs>>) {
	for (mut shaper_inputs, mut painter_inputs) in query.iter_mut() {
		let edge_rect = shaper_inputs.edge_zrect;
		if let (Some(shaper), Some(edge_zrect)) = (&mut shaper_inputs.shaper, &edge_rect) {
			let result = shaper.shape(*edge_zrect);
			match result {
				ShapeResult::NoChange => (),
				ShapeResult::NewPainters(painters) => {
					painter_inputs.painters = painters;
				}
			}
		} else {
			painter_inputs.painters.clear();
		}
	}
}

pub fn apply_painters_update_fills(mut query: Query<(&PainterInputs, &mut MeshInputs), Changed<PainterInputs>>) {
	for (painter_inputs, mut mesh_inputs) in query.iter_mut() {
		let fills = painter_inputs.painters.iter().fold(Vec::new(), |mut fills, painter| {
			fills.extend(painter.paint());
			fills
		});
		if fills != mesh_inputs.fills {
			mesh_inputs.fills = fills;
		}
	}
}

pub fn apply_fills_update_meshes(mut query: Query<(&MeshInputs, &mut MeshOutputs), Changed<MeshInputs>>, app_assets: Res<AppAssets>, mut commands: Commands) {
	for (mesh_inputs, mut mesh_outputs) in query.iter_mut() {
		for entity in &mesh_outputs.mesh_ids {
			commands.entity(*entity).despawn();
		}
		let mut mesh_ids = Vec::new();
		let MeshInputs { fills, max_row } = &mesh_inputs;
		for fill in fills {
			let (color_index, mesh_index) = match &fill.glyph {
				Glyph::Solid(color_index) => (*color_index, 1),
				Glyph::Text(color_index) => (*color_index, 0),
			};
			let id = commands.spawn(MaterialMesh2dBundle {
				mesh: Mesh2dHandle(app_assets.meshes[mesh_index].clone()),
				material: app_assets.color_materials[color_index].clone(),
				transform: {
					let center = Transform::from_xyz(0.5, -0.5, 0.);
					let scale = Transform::from_scale((fill.width(), fill.height(), 1.).into());
					let shift = Transform::from_xyz(fill.left(), *max_row as f32 - fill.top(), fill.near());
					let together = shift.compute_matrix().mul_mat4(&scale.compute_matrix()).mul_mat4(&center.compute_matrix());
					Transform::from_matrix(together)
				},
				..default()
			}).id();
			mesh_ids.push(id);
		}
		mesh_outputs.mesh_ids = mesh_ids;
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
