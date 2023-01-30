use crate::{renderer::Renderer, window::Window};

pub struct App {
	window: Window
}

const APP_NAME: &str = "Cell Life";

impl App {
	pub fn new() -> Self {
		let renderer = Renderer::new();
		let window = Window::new(APP_NAME, Box::new(renderer));
		Self { window }
	}

	pub fn start(self) {
		self.window.run();
	}
}
