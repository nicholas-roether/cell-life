use std::rc::Rc;

use glow::HasContext;

#[derive(Debug, Clone)]
pub struct GraphicsContext {
	pub gl: Rc<glow::Context>
}

impl GraphicsContext {
	pub fn new(gl: Rc<glow::Context>) -> Self {
		Self { gl }
	}

	pub fn init(&self) {
		unsafe {
			self.gl
				.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
			self.gl.enable(glow::BLEND);
		}
	}

	pub fn clear(&self) {
		unsafe {
			self.gl.clear(glow::COLOR_BUFFER_BIT);
		}
	}

	pub fn draw(&self, mode: u32, count: usize) {
		unsafe { self.gl.draw_arrays(mode, 0, count as i32) }
	}
}
