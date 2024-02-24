use bevy::prelude::{ColorMaterial, Component, Mesh, Resource};
use bevy::asset::Handle;

#[derive(Resource)]
pub struct AppAssets {
	pub color_materials: Vec<Handle<ColorMaterial>>,
	pub meshes: Vec<Handle<Mesh>>,
}

#[derive(Component)]
pub struct OrthoCam;
