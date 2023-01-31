use std::rc::Rc;

use glam::{vec2, vec3};

use crate::{
	render::{layers::dots::DotsLayer, Renderer},
	simulation::{Dot, Simulation},
	window::Window
};

pub struct App {
	window: Window
}

const APP_NAME: &str = "Cell Life";

impl App {
	pub fn new() -> Self {
		let window = Window::new(APP_NAME, |gl| {
			let mut renderer = Renderer::new(Rc::clone(&gl));
			renderer.push_layer(DotsLayer::new(
				gl,
				Simulation {
					dots: vec![
						Dot {
							coords: vec2(-30.0, 0.0),
							radius: 5.0,
							color: vec3(1.0, 0.0, 0.0),
							brightness: 10.0
						},
						Dot {
							coords: vec2(30.0, 0.0),
							radius: 8.0,
							color: vec3(0.1, 1.0, 0.2),
							brightness: 0.0
						},
						Dot {
							coords: vec2(0.0, -40.0),
							radius: 3.0,
							color: vec3(0.2, 0.3, 1.0),
							brightness: 20.0
						},
					]
				}
			));
			renderer
		});
		Self { window }
	}

	pub fn start(self) {
		self.window.run();
	}
}
