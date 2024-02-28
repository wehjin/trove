use std::fmt::Debug;
use std::marker::PhantomData;

use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::IntoSystemConfigs;

use crate::resources::solar_dark;
use crate::systems::console::add_console;
use crate::systems::setup::{add_app_assets, setup_camera};
use crate::tools::{ViewStarting, ViewModel};
use crate::tools::views::FabInit;
use crate::view_plugin::systems::{add_root_view, start_views, update_fills, update_focus_options, update_meshes, update_model_queue, update_painters_captors, update_user_queue, update_views};

pub mod components;
pub mod systems;
pub mod tools;

pub struct AlphaPlugin<Seed> {
	seed: PhantomData<Seed>,
}

impl<Seed> Default for AlphaPlugin<Seed>
	where Seed: ViewStarting + Send + Sync + 'static,
{
	fn default() -> Self {
		Self { seed: PhantomData }
	}
}

impl<Seed> Plugin for AlphaPlugin<Seed>
	where Seed: ViewStarting + Send + Sync + 'static,
{
	fn build(&self, app: &mut App) {
		app.insert_resource(solar_dark::PALETTE16)
			.add_systems(Startup, add_console)
			.add_systems(Startup, add_app_assets)
			.add_systems(Startup, setup_camera.after(add_console))
			.add_systems(Startup, add_root_view::<Seed>.after(add_console))
			.add_systems(Update, update_user_queue)
			.add_systems(Update, update_fills)
			.add_systems(Update, update_meshes.after(update_fills))
			.add_plugins(BetaPlugin::<FabInit>::default())
		;
	}
}

pub struct BetaPlugin<Seed> {
	data: PhantomData<Seed>,
}

impl<Seed> Default for BetaPlugin<Seed> where
	Seed: ViewStarting + Send + Sync + 'static,
{
	fn default() -> Self {
		Self { data: PhantomData }
	}
}

impl<Seed> Plugin for BetaPlugin<Seed>
	where Seed: ViewStarting + Send + Sync + 'static,
	      <Seed::Model as ViewModel>::Msg: Copy + Debug
{
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, start_views::<Seed>)
			.add_systems(Update, update_views::<<Seed::Model as ViewModel>::Msg>)
			.add_systems(Update, update_painters_captors::<<Seed::Model as ViewModel>::Msg>)
			.add_systems(Update, update_focus_options::<<Seed::Model as ViewModel>::Msg>)
			.add_systems(Update, update_model_queue::<<Seed::Model as ViewModel>::Msg>)
		;
	}
}
