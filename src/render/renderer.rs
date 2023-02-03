use std::rc::Rc;

use winit::dpi::LogicalSize;

use crate::window;

use super::GraphicsContext;

pub trait Layer {
	fn draw(&mut self, size: LogicalSize<f32>);
}

pub struct Renderer {
	ctx: GraphicsContext,
	layers: Vec<Box<dyn Layer>>
}

impl window::Renderer for Renderer {
	fn draw(&mut self, size: LogicalSize<f32>) {
		self.ctx.clear();

		for layer in &mut self.layers {
			layer.draw(size);
		}
	}
}

impl Renderer {
	pub fn new(gl: Rc<glow::Context>) -> Self {
		let ctx = GraphicsContext::new(gl);
		ctx.init();
		Self {
			ctx,
			layers: Vec::new()
		}
	}

	pub fn push_layer<L: Layer + 'static, F: FnOnce(GraphicsContext) -> L>(
		&mut self,
		layer_builder: F
	) {
		self.layers.push(Box::new(layer_builder(self.ctx.clone())));
	}
}
