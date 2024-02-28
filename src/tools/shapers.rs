use crate::tools::{BoxCaptor, BoxPainter, Shaper, ShaperEffects, ShaperMsg};
use crate::tools::frame::Frame;

pub struct EdgeShaper<Msg> {
	edge_frame: Option<Frame>,
	build_painter: Box<dyn Fn(Frame) -> BoxPainter + Send + Sync + 'static>,
	build_captor: Box<dyn Fn(Frame) -> BoxCaptor<Msg> + Send + Sync + 'static>,
}

impl<Msg> EdgeShaper<Msg> {
	pub fn new(
		build_painter: impl Fn(Frame) -> BoxPainter + Send + Sync + 'static,
		build_captor: impl Fn(Frame) -> BoxCaptor<Msg> + Send + Sync + 'static) -> Self {
		Self { edge_frame: None, build_painter: Box::new(build_painter), build_captor: Box::new(build_captor) }
	}
}

impl<Msg> Shaper<Msg> for EdgeShaper<Msg> {
	fn shape(&mut self, msg: ShaperMsg, effects: &mut ShaperEffects<Msg>) {
		match msg {
			ShaperMsg::SetEdge(edge_frame) => {
				if self.edge_frame != Some(edge_frame) {
					effects.set_painters(vec![(self.build_painter)(edge_frame)]);
					effects.set_captor((self.build_captor)(edge_frame));
				}
			}
		}
	}
}
