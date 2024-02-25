use bevy::prelude::Component;

use crate::tools::Layout;

#[derive(Component)]
pub struct LouterMadeRenderer;

#[derive(Component)]
pub struct Louter {
	pub layout: Box<dyn Layout + Send + Sync>,
}

#[derive(Component)]
pub struct RootLouter;
