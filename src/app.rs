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
		let window = Window::new(APP_NAME, |gl| {
			Renderer::new(
				gl,
				vec![
					Dot {
						coords: Vec2 { x: -30.0, y: 0.0 },
						radius: 5.0,
						color: Vec3 {
							x: 1.0,
							y: 0.0,
							z: 0.0
						},
						brightness: 10.0
					},
					Dot {
						coords: Vec2 { x: 30.0, y: 0.0 },
						radius: 8.0,
						color: Vec3 {
							x: 0.1,
							y: 1.0,
							z: 0.2
						},
						brightness: 0.0
					},
					Dot {
						coords: Vec2 { x: 0.0, y: -40.0 },
						radius: 3.0,
						color: Vec3 {
							x: 0.2,
							y: 0.3,
							z: 1.0
						},
						brightness: 20.0
					},
				]
			)
		});
		Self { window }
	}

	pub fn start(self) {
		self.window.run();
	}
}
