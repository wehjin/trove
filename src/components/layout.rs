use bevy::prelude::Component;
use crate::components::render::Renderer;
use crate::tools::Volume;

#[derive(Component)]
pub struct Louter {
	pub layout: Box<LayoutFn>,
}

pub type LayoutFn = dyn Fn(Volume) -> Vec<Renderer> + Send + Sync;

#[derive(Component)]
pub struct LouterRenderer;

#[derive(Component)]
pub struct RootLouter;
