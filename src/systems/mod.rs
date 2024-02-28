use std::fmt::Debug;

use bevy::prelude::{Bundle, ButtonInput, Changed, Commands, Component, default, Entity, info, KeyCode, Query, Res, ResMut, Transform};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::components::fill::Fill;
use crate::components::setup::AppAssets;
use crate::components::view::{ModelInputs, RootViewMarker};
use crate::RootViewStarter;
use crate::tools::{BoxCaptor, BoxPainter, BoxShaper, Shaper, ShaperEffects, ShaperMsg, ViewStarting};
use crate::tools::console::Console;
use crate::tools::fill::Glyph;
use crate::tools::frame::Frame;

pub mod console;
pub mod setup;

#[derive(Component, Default)]
pub struct ModelOutputs<Msg> {
	shaper: Option<Box<dyn Shaper<Msg> + Send + Sync>>,
}

#[derive(Component, Default)]
pub struct ShaperInputs {
	shaper_count: usize,
	edge_frame: Option<Frame>,
}

#[derive(Component, Default)]
pub struct CaptorInputs<Msg> {
	captor: Option<BoxCaptor<Msg>>,
}

impl<Msg> CaptorInputs<Msg> {
	pub fn to_space_msg(&self, pressed: bool) -> Option<Msg> {
		if let Some(captor) = &self.captor {
			captor.to_space_msg(pressed)
		} else {
			None
		}
	}
}


#[derive(Component, Default)]
pub struct PainterInputs {
	painters: Vec<BoxPainter>,
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
struct ViewBundle<Msg: Send + Sync + 'static> {
	model_inputs: ModelInputs<Msg>,
	model_outputs: ModelOutputs<Msg>,
	shaper_inputs: ShaperInputs,
	painter_inputs: PainterInputs,
	mesh_inputs: MeshInputs,
	mesh_outputs: MeshOutputs,
	captor_inputs: CaptorInputs<Msg>,
}

pub struct ViewEffects<'a, 'w, 's, Msg> {
	pub commands: &'a mut Commands<'w, 's>,
	pub new_shaper: Option<BoxShaper<Msg>>,
}

impl<'a, 'w, 's, Msg> ViewEffects<'a, 'w, 's, Msg> {
	pub fn set_shaper(&mut self, shaper: impl Shaper<Msg> + Send + Sync + 'static) {
		self.new_shaper = Some(Box::new(shaper));
	}
}

pub fn add_root_view<T: ViewStarting + Send + Sync + 'static>(console: Res<Console>, mut starter: ResMut<RootViewStarter<T>>, mut commands: Commands) {
	info!("add_root_view");
	let (cols, rows) = console.width_height();
	let mut effects = ViewEffects { commands: &mut commands, new_shaper: None };
	let model = starter.value.take().expect("root view starter").start_view(&mut effects);
	let model_inputs = ModelInputs {
		model: Box::new(model),
		msg_queue: Vec::new(),
	};
	let model_outputs = ModelOutputs {
		shaper: effects.new_shaper,
	};
	let shaper_inputs = ShaperInputs {
		shaper_count: 1,
		edge_frame: Some(Frame::from_cols_rows_z(cols, rows, 1)),
	};
	let painter_inputs = PainterInputs { painters: Vec::new() };
	let mesh_inputs = MeshInputs { fills: Vec::new(), max_row: rows };
	let mesh_outputs = MeshOutputs::default();
	let captor_inputs = CaptorInputs { captor: None };
	let bundle = ViewBundle {
		model_inputs,
		model_outputs,
		shaper_inputs,
		painter_inputs,
		mesh_inputs,
		mesh_outputs,
		captor_inputs,
	};
	commands.spawn((RootViewMarker, bundle));
}

pub fn update_models<Msg: Send + Sync + 'static + Debug>(
	mut query: Query<(
		&mut ModelInputs<Msg>,
		&mut ModelOutputs<Msg>,
		&mut ShaperInputs
	), Changed<ModelInputs<Msg>>>, mut commands: Commands) {
	for (mut model_inputs, mut model_outputs, mut shaper_inputs) in query.iter_mut() {
		info!("update_models");
		if model_inputs.msg_queue.is_empty() {
			// DO NOT touch msg_queue mutable with pop if it is empty.
			info!("  no msg in queue");
			continue;
		}
		if let Some(msg) = model_inputs.msg_queue.pop() {
			info!("  msg: {:?}", msg);
			let mut effects = ViewEffects { commands: &mut commands, new_shaper: None };
			model_inputs.model.update_view(msg, &mut effects);
			if let Some(shaper) = effects.new_shaper {
				model_outputs.shaper = Some(shaper);
				shaper_inputs.shaper_count += 1;
			}
		}
	}
}

pub fn update_painters_captors<Msg: Send + Sync + 'static>(
	mut query: Query<
		(&ShaperInputs, &mut ModelOutputs<Msg>, &mut PainterInputs, &mut CaptorInputs<Msg>),
		Changed<ShaperInputs>
	>
) {
	for (shaper_inputs, mut model_outputs, mut painter_inputs, mut captor_inputs) in query.iter_mut() {
		info!("update_painters_captors");
		let shaper_inputs_edge_frame = shaper_inputs.edge_frame;
		if let Some(shaper) = &mut model_outputs.shaper {
			info!("  have shaper");
			if let Some(edge_frame) = shaper_inputs_edge_frame {
				info!("  have edge_frame");
				let mut effects = ShaperEffects::default();
				shaper.shape(ShaperMsg::SetEdge(edge_frame), &mut effects);
				if let Some(painters) = effects.new_painters {
					info!("  have painters: {}", painters.len());
					painter_inputs.painters = painters;
				}
				if let Some(captor) = effects.new_captor {
					info!("  have captor");
					captor_inputs.captor = Some(captor);
				}
			}
		}
	}
}

pub fn keyboard_input<Msg: Send + Sync + 'static>(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<(&CaptorInputs<Msg>, &mut ModelInputs<Msg>)>) {
	if keyboard_input.just_pressed(KeyCode::Space) {
		info!("' ' just pressed");
		for (captor_input, mut model_inputs) in query.iter_mut() {
			let space_msg = captor_input.to_space_msg(true);
			if let Some(msg) = space_msg {
				model_inputs.msg_queue.push(msg)
			}
		}
	}
	if keyboard_input.just_released(KeyCode::Space) {
		info!("' ' just released");
		for (captor_input, mut model_inputs) in query.iter_mut() {
			let space_msg = captor_input.to_space_msg(false);
			if let Some(msg) = space_msg {
				model_inputs.msg_queue.push(msg)
			}
		}
	}
}

pub fn update_fills(mut query: Query<(&PainterInputs, &mut MeshInputs), Changed<PainterInputs>>) {
	for (painter_inputs, mut mesh_inputs) in query.iter_mut() {
		info!("update_fills");
		let painter_count = painter_inputs.painters.len();
		info!("  painter-count: {painter_count}");
		let fills = painter_inputs.painters.iter().fold(Vec::new(), |mut fills, painter| {
			fills.extend(painter.paint());
			fills
		});
		let fill_count = fills.len();
		info!("  fill-count: {fill_count}");
		if fills != mesh_inputs.fills {
			mesh_inputs.fills = fills;
		}
	}
}

pub fn update_meshes(mut query: Query<(&MeshInputs, &mut MeshOutputs), Changed<MeshInputs>>, app_assets: Res<AppAssets>, mut commands: Commands) {
	for (mesh_inputs, mut mesh_outputs) in query.iter_mut() {
		for entity in &mesh_outputs.mesh_ids {
			commands.entity(*entity).despawn();
		}
		let mut mesh_ids = Vec::new();
		let MeshInputs { fills, max_row } = &mesh_inputs;
		let fill_count = fills.len();
		info!("update_meshes, fill count: {fill_count}");
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
