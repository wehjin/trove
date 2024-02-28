use std::collections::HashSet;
use std::fmt::Debug;

use bevy::input::ButtonInput;
use bevy::log::info;
use bevy::prelude::{Changed, Commands, default, KeyCode, Query, Res, ResMut, Transform};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::components::setup::AppAssets;
use crate::tools::{ShaperEffects, ShaperMsg, UserEvent, ViewModel, ViewStarting};
use crate::tools::console::Console;
use crate::tools::fill::Glyph;
use crate::tools::frame::Frame;
use crate::view_plugin::components::{CaptorInputs, FocusOptions, MeshInputs, MeshOutputs, ViewProcess, ModelOutputs, PainterInputs, RootViewMarker, ShaperInputs, UserEventQueue, ViewSeed};
use crate::view_plugin::tools::{ViewBundle, ViewEffects};
use crate::view_plugin::tools::RootViewStarter;

pub fn add_root_view<T: ViewStarting + Send + Sync + 'static>(mut root_seed: ResMut<RootViewStarter<T>>, mut commands: Commands) {
	info!("add_root_view");
	let seed = root_seed.value.take().expect("root-view seed");
	let bundle = ViewBundle {
		view_seed: ViewSeed { value: Some(Box::new(seed)) },
		..ViewBundle::<T>::default()
	};
	commands.spawn((RootViewMarker, bundle));
}

pub fn start_views<T: ViewStarting + Send + Sync + 'static>(
	console: Res<Console>,
	mut query: Query<(
		&mut ViewSeed<T>,
		&mut ViewProcess<<T::Model as ViewModel>::Msg>,
		&mut ModelOutputs<<T::Model as ViewModel>::Msg>,
		&mut ShaperInputs,
		&mut MeshInputs,
		Option<&RootViewMarker>
	), Changed<ViewSeed<T>>>)
{
	for (
		mut view_seed,
		mut model_inputs,
		mut model_outputs,
		mut shaper_inputs,
		mut mesh_inputs,
		root_view_marker
	) in query.iter_mut() {
		if view_seed.value.is_none() {
			// DO NOT take from comp if no seed, avoids an infinite loop where the comp is changed
			// every time this system is run.
			continue;
		}
		let seed = view_seed.value.take().expect("seed");
		{
			let mut effects = ViewEffects { new_shaper: None };
			let model = seed.init_view_model(&mut effects);
			model_inputs.model = Some(Box::new(model));
			if let Some(shaper) = effects.new_shaper {
				model_outputs.shaper = Some(shaper);
				shaper_inputs.shaper_count += 1;
			}
		}
		{
			let (cols, rows) = console.width_height();
			mesh_inputs.max_row = rows;
			if root_view_marker.is_some() {
				shaper_inputs.edge_frame = Some(Frame::from_cols_rows_z(cols, rows, 1));
			}
		}
	}
}

pub fn update_views<Msg: Send + Sync + 'static + Debug>(
	mut query: Query<(
		&mut ViewProcess<Msg>,
		&mut ModelOutputs<Msg>,
		&mut ShaperInputs
	), Changed<ViewProcess<Msg>>>
) {
	for (mut model_inputs, mut model_outputs, mut shaper_inputs) in query.iter_mut() {
		info!("update_models");
		if model_inputs.model.is_none() {
			continue;
		}
		let mut model = model_inputs.model.take().expect("some model");
		if model_inputs.msg_queue.is_empty() {
			// DO NOT touch msg_queue mutable with pop if it is empty.
			info!("  no msg in queue");
			continue;
		}
		if let Some(msg) = model_inputs.msg_queue.pop() {
			info!("  msg: {:?}", msg);
			let mut effects = ViewEffects { new_shaper: None };
			model.update_as_view_model(msg, &mut effects);
			model_inputs.model = Some(model);
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

pub fn update_focus_options<Msg: Send + Sync + 'static>(
	mut query: Query<(&CaptorInputs<Msg>, &mut FocusOptions), Changed<CaptorInputs<Msg>>>
) {
	for (captor_inputs, mut focus_options) in query.iter_mut() {
		if let Some(captor) = &captor_inputs.captor {
			let captures = captor.event_map.iter().map(|(k, _)| *k).collect::<HashSet<_>>();
			focus_options.captures = captures;
		} else {
			focus_options.captures = HashSet::new();
		}
	}
}

pub fn update_user_queue(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<(&FocusOptions, &mut UserEventQueue)>) {
	for (focus_options, mut queue) in query.iter_mut() {
		let mut new_queue = Vec::new();
		if keyboard_input.just_pressed(KeyCode::Space) {
			info!("' ' just pressed");
			let user_event = UserEvent::PressStart;
			if focus_options.captures.contains(&user_event) {
				new_queue.push(user_event);
			}
		}
		if keyboard_input.just_released(KeyCode::Space) {
			info!("' ' just released");
			let user_event = UserEvent::PressEnd;
			if focus_options.captures.contains(&user_event) {
				new_queue.push(user_event);
			}
		}
		if !new_queue.is_empty() {
			queue.user_events = new_queue;
		}
	}
}

pub fn update_model_queue<Msg: Copy + Send + Sync + 'static>(
	mut query: Query<(&UserEventQueue, &CaptorInputs<Msg>, &mut ViewProcess<Msg>), Changed<UserEventQueue>>
) {
	for (user_queue, captor_inputs, mut model_inputs) in query.iter_mut() {
		if let Some(captor) = &captor_inputs.captor {
			for event in &user_queue.user_events {
				if let Some(msg) = captor.get_msg(event) {
					model_inputs.msg_queue.push(msg);
				}
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
