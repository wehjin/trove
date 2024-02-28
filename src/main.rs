use std::error::Error;

use bevy::DefaultPlugins;
use bevy::prelude::{App, IntoSystemConfigs, Startup};

use systems::add_circles;
use systems::console::{add_console, add_panels};
use systems::setup::add_app_assets;
use tools::console::Console;
use tools::sample::SampleAppSettings;
use view_plugin::AlphaPlugin;
use view_plugin::tools::RootViewStarter;

use crate::tools::views::FabInit;

pub mod components;
pub mod view_plugin;
pub mod resources;
pub mod systems;
pub mod tools;

fn main() -> Result<(), Box<dyn Error>> {
	Console::start()?;
	let _root_view_starter = RootViewStarter { value: Some(SampleAppSettings) };
	let root_view_starter = RootViewStarter { value: Some(FabInit::default()) };
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(AlphaPlugin)
		.add_systems(Startup, add_panels.after(add_console))
		.add_systems(Startup, add_circles.after(add_console).after(add_app_assets))
		.insert_resource(root_view_starter)
		.run();
	Console::stop()?;
	Ok(())
}
