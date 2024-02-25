use bevy::prelude::Component;

use crate::tools::LayoutFn;

#[derive(Component)]
pub struct LouterMadeRenderer;

#[derive(Component)]
pub struct Louter {
	pub layout: Box<LayoutFn>,
}

#[derive(Component)]
pub struct RootLouter;
