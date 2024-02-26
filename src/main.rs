use std::error::Error;

use bevy::DefaultPlugins;
use bevy::prelude::{App, IntoSystemConfigs, Resource, Startup, Update};

use systems::add_circles;
use systems::console::{add_console, add_panels, flush_console, greet_panels, hello_world};
use systems::setup::{add_app_assets, setup_camera};
use tools::console::Console;
use tools::sample::SampleAppSettings;
use tools::ViewBuilding;

use crate::resources::solar_dark;
use crate::systems::{add_root_view, apply_fills_update_meshes, apply_painters_update_fills, apply_shapers_update_painters};

pub mod components;
pub mod resources;
pub mod systems;
pub mod tools;

#[derive(Resource)]
pub struct RootViewBuilder<T: ViewBuilding> {
	pub value: Option<T>,
}

fn main() -> Result<(), Box<dyn Error>> {
	Console::start()?;
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(solar_dark::PALETTE16)
		.insert_resource(RootViewBuilder { value: Some(SampleAppSettings) })
		.add_systems(Startup, add_console)
		.add_systems(Startup, add_app_assets)
		.add_systems(Startup, setup_camera.after(add_console))
		.add_systems(Startup, add_panels.after(add_console))
		.add_systems(Startup, add_circles.after(add_console).after(add_app_assets))
		.add_systems(Startup, add_root_view.after(add_console))

		.add_systems(Update, flush_console)
		.add_systems(Update, hello_world.before(flush_console))
		.add_systems(Update, greet_panels.after(hello_world).before(flush_console))
		.add_systems(Update, apply_shapers_update_painters)
		.add_systems(Update, apply_painters_update_fills.after(apply_shapers_update_painters))
		.add_systems(Update, apply_fills_update_meshes.after(apply_painters_update_fills))
		.run();
	Console::stop()?;
	Ok(())
}
