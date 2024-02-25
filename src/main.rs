use std::error::Error;

use bevy::DefaultPlugins;
use bevy::prelude::{App, IntoSystemConfigs, Startup, Update};

use systems::add_circles;
use systems::console::{add_console, add_panels, flush_console, greet_panels, hello_world};
use systems::fill::{despawn_fill_meshes, spawn_fill_meshes};
use systems::layout::{despawn_layout_renderings, spawn_root_layout_renderings};
use systems::render::{despawn_rendering_fills, spawn_rendering_fills};
use systems::setup::{add_app_assets, setup_camera};
use systems::view::{despawn_volume_renderers, spawn_root_view_volume_renderers};
use tools::console::Console;

use crate::resources::solar_dark;
use crate::systems::add_root_view;

mod components;
mod resources;
mod systems;
mod tools;

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
		.add_systems(Startup, add_root_view)

		.add_systems(Update, flush_console)
		.add_systems(Update, hello_world.before(flush_console))
		.add_systems(Update, greet_panels.after(hello_world).before(flush_console))
		.add_systems(Update, despawn_fill_meshes)
		.add_systems(Update, despawn_rendering_fills.after(despawn_fill_meshes))
		.add_systems(Update, despawn_layout_renderings.after(despawn_rendering_fills))
		.add_systems(Update, despawn_volume_renderers.after(despawn_layout_renderings))
		.add_systems(Update, spawn_root_view_volume_renderers::<()>.after(despawn_volume_renderers))
		.add_systems(Update, spawn_root_layout_renderings.after(spawn_root_view_volume_renderers::<()>))
		.add_systems(Update, spawn_rendering_fills.after(spawn_root_layout_renderings))
		.add_systems(Update, spawn_fill_meshes.after(spawn_rendering_fills))
		.run();
	Console::stop()?;
	Ok(())
}
