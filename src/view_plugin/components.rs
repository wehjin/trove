use std::collections::HashSet;

use bevy::prelude::{Component, Entity};

use crate::components::fill::Fill;
use crate::tools::{BoxPainter, Captor, Shaper, UserEvent, ViewModel, ViewStarting};
use crate::tools::frame::Frame;

#[derive(Component)]
pub struct ViewSeed<T: ViewStarting + Send + Sync + 'static> {
	pub value: Option<Box<T>>,
}

#[derive(Component)]
pub struct ViewProcess<Msg> {
	pub model: Option<Box<dyn ViewModel<Msg=Msg> + Send + Sync>>,
	pub msg_queue: Vec<Msg>,
}

#[derive(Component)]
pub struct ModelOutputs<Msg> {
	pub shaper: Option<Box<dyn Shaper<Msg> + Send + Sync>>,
}

#[derive(Component, Default)]
pub struct ShaperInputs {
	pub shaper_count: usize,
	pub edge_frame: Option<Frame>,
}

#[derive(Component)]
pub struct CaptorInputs<Msg> {
	pub captor: Option<Captor<Msg>>,
}

#[derive(Component, Default)]
pub struct FocusOptions {
	pub captures: HashSet<UserEvent>,
}

#[derive(Component, Default)]
pub struct UserEventQueue {
	pub user_events: Vec<UserEvent>,
}

#[derive(Component, Default)]
pub struct PainterInputs {
	pub painters: Vec<BoxPainter>,
}

#[derive(Component, Default)]
pub struct MeshInputs {
	pub fills: Vec<Fill>,
	pub max_row: u16,
}

#[derive(Component, Default)]
pub struct MeshOutputs {
	pub mesh_ids: Vec<Entity>,
}

#[derive(Component)]
pub struct RootViewMarker;
