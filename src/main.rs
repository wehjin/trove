use std::error::Error;

use bevy::DefaultPlugins;
use bevy::prelude::{App, IntoSystemConfigs, Startup, Update};

use console::Console;

fn main() -> Result<(), Box<dyn Error>> {
	Console::start()?;
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, systems::add_console)
		.add_systems(Startup, systems::add_palette)
		.add_systems(Startup, systems::setup_camera.after(systems::add_console))
		.add_systems(Startup, systems::add_panels.after(systems::add_console))
		.add_systems(Startup, systems::add_circles.after(systems::add_console).after(systems::add_palette))
		.add_systems(Update, systems::flush_console)
		.add_systems(Update, systems::hello_world.before(systems::flush_console))
		.add_systems(Update, systems::greet_panels.after(systems::hello_world).before(systems::flush_console))
		.run();
	Console::stop()?;
	Ok(())
}

mod components;
pub mod console;
mod systems;
