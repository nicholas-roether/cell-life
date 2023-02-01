use glam::{vec2, Vec2, Vec3};

use crate::render::layers;

use super::Tick;

#[derive(Debug)]
pub struct Cell {
	center: Vec2,
	coords: Vec2,
	radius: f32,
	color: Vec3,
	brightness: f32,
	time: f64
}

impl Cell {
	pub fn new(coords: Vec2, radius: f32, color: Vec3, brightness: f32) -> Self {
		Self {
			center: coords,
			coords,
			radius,
			color,
			brightness,
			time: color.x as f64
		}
	}
}

impl Tick for Cell {
	fn tick(&mut self, dt: f64) {
		self.time += dt;
		self.coords = self.center + vec2(self.time.cos() as f32, self.time.sin() as f32) * 20.0;
		self.brightness = ((self.time * 1.2).sin().powi(2) * 10.0) as f32
	}
}

impl From<&Cell> for layers::dots::Dot {
	fn from(value: &Cell) -> Self {
		layers::dots::Dot {
			coords: value.coords,
			radius: value.radius,
			color: value.color,
			brightness: value.brightness
		}
	}
}
