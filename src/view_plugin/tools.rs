use bevy::prelude::{Bundle, Commands, Resource};
use crate::components::view::ModelInputs;
use crate::tools::{BoxShaper, Shaper, ViewStarting};
use crate::view_plugin::components::{CaptorInputs, FocusOptions, MeshInputs, MeshOutputs, ModelOutputs, PainterInputs, ShaperInputs, UserEventQueue};

#[derive(Bundle)]
pub struct ViewBundle<Msg: Send + Sync + 'static> {
	pub model_inputs: ModelInputs<Msg>,
	pub model_outputs: ModelOutputs<Msg>,
	pub shaper_inputs: ShaperInputs,
	pub painter_inputs: PainterInputs,
	pub mesh_inputs: MeshInputs,
	pub mesh_outputs: MeshOutputs,
	pub captor_inputs: CaptorInputs<Msg>,
	pub focus_options: FocusOptions,
	pub user_event_queue: UserEventQueue,
}

pub struct ViewEffects<'a, 'w, 's, Msg> {
	pub commands: &'a mut Commands<'w, 's>,
	pub new_shaper: Option<BoxShaper<Msg>>,
}

impl<'a, 'w, 's, Msg> ViewEffects<'a, 'w, 's, Msg> {
	pub fn set_shaper(&mut self, shaper: impl Shaper<Msg> + Send + Sync + 'static) {
		self.new_shaper = Some(Box::new(shaper));
	}
}

#[derive(Resource)]
pub struct RootViewStarter<T: ViewStarting + Send + Sync + 'static> {
	pub value: Option<T>,
}
