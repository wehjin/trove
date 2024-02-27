use std::error::Error;

use bevy::DefaultPlugins;
use bevy::prelude::{App, IntoSystemConfigs, Resource, Startup, Update};

use systems::add_circles;
use systems::console::{add_console, add_panels};
use systems::setup::{add_app_assets, setup_camera};
use tools::console::Console;
use tools::sample::SampleAppSettings;
use tools::ViewStarting;

use crate::resources::solar_dark;
use crate::systems::{add_root_view, apply_fills_update_meshes, apply_painters_update_fills, apply_shapers_update_painters};
use crate::tools::views::FabInit;

pub mod components;
pub mod resources;
pub mod systems;
pub mod tools;

#[derive(Resource)]
pub struct RootViewStarter<T: ViewStarting + Send + Sync + 'static> {
	pub value: Option<T>,
}

fn main() -> Result<(), Box<dyn Error>> {
	Console::start()?;
	let _root_view_starter = RootViewStarter { value: Some(SampleAppSettings) };
	let root_view_starter = RootViewStarter { value: Some(FabInit::default()) };
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(solar_dark::PALETTE16)
		.insert_resource(root_view_starter)
		.add_systems(Startup, add_console)
		.add_systems(Startup, add_app_assets)
		.add_systems(Startup, setup_camera.after(add_console))
		.add_systems(Startup, add_panels.after(add_console))
		.add_systems(Startup, add_circles.after(add_console).after(add_app_assets))
		.add_systems(Startup, add_root_view::<FabInit>.after(add_console))
		.add_systems(Update, apply_shapers_update_painters)
		.add_systems(Update, apply_painters_update_fills.after(apply_shapers_update_painters))
		.add_systems(Update, apply_fills_update_meshes.after(apply_painters_update_fills))
		.run();
	Console::stop()?;
	Ok(())
}
