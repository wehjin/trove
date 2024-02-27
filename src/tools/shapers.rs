use crate::tools::{BoxPainter, Shaper, ShaperEffects, ShaperMsg};
use crate::tools::frame::Frame;

pub struct PainterShaper {
	edge_frame: Option<Frame>,
	painter_builder: Box<dyn Fn(Frame) -> BoxPainter + Send + Sync + 'static>,
}

impl PainterShaper {
	pub fn new(painter_builder: impl Fn(Frame) -> BoxPainter + Send + Sync + 'static) -> Self {
		Self { edge_frame: None, painter_builder: Box::new(painter_builder) }
	}
}

impl Shaper for PainterShaper {
	fn shape(&mut self, msg: ShaperMsg, effects: &mut ShaperEffects) {
		match msg {
			ShaperMsg::SetEdge(edge_frame) => {
				if self.edge_frame != Some(edge_frame) {
					let painter = (self.painter_builder)(edge_frame);
					effects.set_painters(vec![painter])
				}
			}
		}
	}
}
