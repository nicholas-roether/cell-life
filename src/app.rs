use crate::{
	renderer::Renderer,
	window::{Window, WindowStatus}
};

pub struct App {
	window: Window,
	renderer: Renderer,
	running: bool
}

const APP_NAME: &str = "Cell Life";

impl App {
	pub fn new() -> Self {
		let window = Window::new(APP_NAME);
		let renderer = Renderer::new(window.gl_context());
		Self {
			window,
			renderer,
			running: false
		}
	}

	pub fn start(&mut self) {
		self.running = true;
		loop {
			if !self.running {
				break;
			}
			self.handle_window_events();
			self.draw();
		}
	}

	pub fn stop(&mut self) {
		self.running = false;
	}

	fn handle_window_events(&mut self) {
		let window_status = self.window.handle_events();
		if window_status == WindowStatus::Closed {
			self.stop();
		}
	}

	fn draw(&mut self) {
		self.renderer.draw();
		self.window.swap();
	}
}
