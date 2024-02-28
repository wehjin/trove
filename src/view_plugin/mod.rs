use std::fmt::Debug;
use std::marker::PhantomData;

use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::IntoSystemConfigs;

use crate::resources::solar_dark;
use crate::systems::console::add_console;
use crate::systems::setup::{add_app_assets, setup_camera};
use crate::tools::views::{FabInit, FabMsg};
use crate::tools::ViewStarting;
use crate::view_plugin::systems::{add_root_view, update_fills, update_focus_options, update_meshes, update_model_queue, update_models, update_painters_captors, update_user_queue};

pub mod components;
pub mod systems;
pub mod tools;

pub struct AlphaPlugin;

impl Plugin for AlphaPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(solar_dark::PALETTE16)
			.add_systems(Startup, add_console)
			.add_systems(Startup, add_app_assets)
			.add_systems(Startup, setup_camera.after(add_console))
			.add_systems(Update, update_user_queue)
			.add_systems(Update, update_fills)
			.add_systems(Update, update_meshes.after(update_fills))
			.add_plugins(BetaPlugin::<FabInit, FabMsg>::default())
		;
	}
}

pub struct BetaPlugin<Seed, Msg> {
	seed: PhantomData<Seed>,
	msg: PhantomData<Msg>,
}

impl<Seed, Msg> Default for BetaPlugin<Seed, Msg> where
	Seed: ViewStarting + Send + Sync + 'static,
	Msg: Copy + Send + Sync + 'static + Debug,
{
	fn default() -> Self {
		Self { seed: PhantomData, msg: PhantomData }
	}
}

impl<Seed, Msg> Plugin for BetaPlugin<Seed, Msg> where
	Seed: ViewStarting + Send + Sync + 'static,
	Msg: Copy + Send + Sync + 'static + Debug,
{
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, add_root_view::<Seed>.after(add_console))
			.add_systems(Update, update_model_queue::<Msg>)
			.add_systems(Update, update_models::<Msg>)
			.add_systems(Update, update_painters_captors::<Msg>)
			.add_systems(Update, update_focus_options::<Msg>)
		;
	}
}
