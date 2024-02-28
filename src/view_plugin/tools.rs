use bevy::prelude::{Bundle, Resource};

use crate::view_plugin::components::ViewProcess;
use crate::tools::{BoxShaper, Shaper, ViewStarting, ViewModel};
use crate::view_plugin::components::{CaptorInputs, FocusOptions, MeshInputs, MeshOutputs, ModelOutputs, PainterInputs, ShaperInputs, UserEventQueue};
use crate::view_plugin::components::ViewSeed;

#[derive(Bundle)]
pub struct ViewBundle<T: ViewStarting + Send + Sync + 'static> {
	pub view_seed: ViewSeed<T>,
	pub model_inputs: ViewProcess<<T::Model as ViewModel>::Msg>,
	pub model_outputs: ModelOutputs<<T::Model as ViewModel>::Msg>,
	pub shaper_inputs: ShaperInputs,
	pub painter_inputs: PainterInputs,
	pub mesh_inputs: MeshInputs,
	pub mesh_outputs: MeshOutputs,
	pub captor_inputs: CaptorInputs<<T::Model as ViewModel>::Msg>,
	pub focus_options: FocusOptions,
	pub user_event_queue: UserEventQueue,
}

impl<T: ViewStarting + Send + Sync + 'static> Default for ViewBundle<T> {
	fn default() -> Self {
		Self {
			view_seed: ViewSeed { value: None },
			model_inputs: ViewProcess { model: None, msg_queue: vec![] },
			model_outputs: ModelOutputs { shaper: None },
			shaper_inputs: Default::default(),
			painter_inputs: Default::default(),
			mesh_inputs: Default::default(),
			mesh_outputs: Default::default(),
			captor_inputs: CaptorInputs { captor: None },
			focus_options: Default::default(),
			user_event_queue: Default::default(),
		}
	}
}

pub struct ViewEffects<Msg> {
	pub new_shaper: Option<BoxShaper<Msg>>,
}

impl<Msg> ViewEffects<Msg> {
	pub fn set_shaper(&mut self, shaper: impl Shaper<Msg> + Send + Sync + 'static) {
		self.new_shaper = Some(Box::new(shaper));
	}
}

#[derive(Resource)]
pub struct RootViewStarter<T: ViewStarting + Send + Sync + 'static> {
	pub value: Option<T>,
}
