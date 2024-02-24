use std::error::Error;

use bevy::DefaultPlugins;
use bevy::prelude::{App, IntoSystemConfigs, Startup, Update};

use console::Console;
use systems::{add_app_assets, add_circles, add_console, add_panels, flush_console, greet_panels, hello_world, setup_camera};

use crate::resources::Palette16;
use crate::systems::{add_fills, despawn_fill_meshes, spawn_fill_meshes};

fn main() -> Result<(), Box<dyn Error>> {
	Console::start()?;
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(Palette16::SOLAR_DARK)
		.add_systems(Startup, add_console)
		.add_systems(Startup, add_app_assets)
		.add_systems(Startup, setup_camera.after(add_console))
		.add_systems(Startup, add_panels.after(add_console))
		.add_systems(Startup, add_circles.after(add_console).after(add_app_assets))
		.add_systems(Startup, add_fills.after(add_console))
		.add_systems(Update, flush_console)
		.add_systems(Update, hello_world.before(flush_console))
		.add_systems(Update, greet_panels.after(hello_world).before(flush_console))
		.add_systems(Update, despawn_fill_meshes)
		.add_systems(Update, spawn_fill_meshes.after(despawn_fill_meshes))
		.run();
	Console::stop()?;
	Ok(())
}

mod components;
mod console;
mod resources;
mod systems;
