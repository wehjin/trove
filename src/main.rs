use std::error::Error;

use bevy::DefaultPlugins;
use bevy::prelude::{App, IntoSystemConfigs, Startup, Update};

use systems::add_circles;
use systems::console::{add_console, add_panels, flush_console, greet_panels, hello_world};
use systems::setup::{add_app_assets, setup_camera};
use tools::console::Console;

use crate::resources::solar_dark;
use crate::systems::{add_root_renderer, despawn_fill_meshes, despawn_renderer_fills, spawn_fill_meshes, spawn_renderer_fills};

fn main() -> Result<(), Box<dyn Error>> {
	Console::start()?;
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(solar_dark::PALETTE16)
		.add_systems(Startup, add_console)
		.add_systems(Startup, add_app_assets)
		.add_systems(Startup, setup_camera.after(add_console))
		.add_systems(Startup, add_panels.after(add_console))
		.add_systems(Startup, add_circles.after(add_console).after(add_app_assets))
		.add_systems(Startup, add_root_renderer.after(add_console))
		.add_systems(Update, flush_console)
		.add_systems(Update, hello_world.before(flush_console))
		.add_systems(Update, greet_panels.after(hello_world).before(flush_console))
		.add_systems(Update, despawn_fill_meshes)
		.add_systems(Update, despawn_renderer_fills.after(despawn_fill_meshes))
		.add_systems(Update, spawn_renderer_fills.after(despawn_renderer_fills))
		.add_systems(Update, spawn_fill_meshes.after(spawn_renderer_fills))
		.run();
	Console::stop()?;
	Ok(())
}

mod components;
mod resources;
mod systems;
mod tools;
