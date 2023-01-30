use crevice::std430::Vec2;

use crate::{
	renderer::{Dot, Renderer},
	window::Window
};

pub struct App {
	window: Window
}

const APP_NAME: &str = "Cell Life";

impl App {
	pub fn new() -> Self {
		let renderer = Renderer::new(vec![Dot {
			coords: Vec2::,
			radius: 5.0,
			color: [1.0, 0.0, 0.0],
			brightness: 0.5
		}]);
		let window = Window::new(APP_NAME, Box::new(renderer));
		Self { window }
	}

	pub fn start(self) {
		self.window.run();
	}
}
