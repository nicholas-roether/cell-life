use std::rc::Rc;

use glow::HasContext;
use winit::dpi::LogicalSize;

use crate::window;

pub trait Layer {
	fn draw(&mut self, size: LogicalSize<f32>);
}

pub struct Renderer {
	gl: Rc<glow::Context>,
	layers: Vec<Box<dyn Layer>>
}

impl window::Renderer for Renderer {
	fn draw(&mut self, size: LogicalSize<f32>) {
		self.clear();

		for layer in &mut self.layers {
			layer.draw(size);
		}
	}
}

impl Renderer {
	pub fn new(gl: Rc<glow::Context>) -> Self {
		Self::config(&gl);

		Self {
			gl,
			layers: Vec::new()
		}
	}

	pub fn push_layer<L: Layer + 'static>(&mut self, layer: L) {
		self.layers.push(Box::new(layer));
	}

	fn clear(&self) {
		unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT) }
	}

	fn config(gl: &glow::Context) {
		unsafe {
			gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
			gl.enable(glow::BLEND);
		}
	}
}
