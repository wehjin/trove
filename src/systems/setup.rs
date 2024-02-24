use bevy::prelude::{Camera2dBundle, Circle, ColorMaterial, Commands, default, Mesh, Rectangle, Res, ResMut};
use bevy::asset::Assets;
use bevy::render::camera::ScalingMode;
use crate::components::{AppAssets, OrthoCam};
use crate::resources::Palette16;
use crate::tools::console::Console;

pub fn setup_camera(mut commands: Commands, console: Res<Console>) {
	let (width, height) = console.width_height();
	let mut camera = Camera2dBundle { ..default() };
	camera.projection.scaling_mode = ScalingMode::Fixed { width: width as f32, height: height as f32 };
	camera.projection.viewport_origin = (0., 0.).into();
	commands.spawn((OrthoCam, camera));
}

pub fn add_app_assets(
	palette: Res<Palette16>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut commands: Commands,
) {
	let app_assets = AppAssets {
		color_materials: palette.to_colors()
			.into_iter()
			.map(|c| materials.add(c))
			.collect::<Vec<_>>(),
		meshes: vec![
			meshes.add(Circle { radius: 0.30 }),
			meshes.add(Rectangle::new(1.0, 1.0)),
		],
	};
	commands.insert_resource(app_assets);
}

