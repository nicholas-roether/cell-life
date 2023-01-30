use crevice::std430::{Vec2, Vec3};

use crate::{
	render::{Dot, Renderer},
	window::Window
};

pub struct App {
	window: Window
}

const APP_NAME: &str = "Cell Life";

impl App {
	pub fn new() -> Self {
		let renderer = Renderer::new(vec![Dot {
			coords: Vec2 { x: 0.0, y: 0.0 },
			radius: 5.0,
			color: Vec3 {
				x: 1.0,
				y: 0.0,
				z: 0.0
			},
			brightness: 0.5
		}]);
		let window = Window::new(APP_NAME, Box::new(renderer));
		Self { window }
	}

	pub fn start(self) {
		self.window.run();
	}
}
