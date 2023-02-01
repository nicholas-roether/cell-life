use glam::{vec2, vec3, Vec2, Vec3};

use crate::render::{layers, ObjectProvider};

pub trait Tick {
	fn tick(&mut self, dt: f64);
}

#[derive(Debug)]
struct Cell {
	center: Vec2,
	coords: Vec2,
	radius: f32,
	color: Vec3,
	brightness: f32,
	time: f64
}

impl Cell {
	fn new(coords: Vec2, radius: f32, color: Vec3, brightness: f32) -> Self {
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
	}
}

#[derive(Debug)]
pub struct Simulation {
	cells: Vec<Cell>
}

impl Simulation {
	pub fn new() -> Self {
		Self {
			cells: vec![
				Cell::new(vec2(-30.0, 0.0), 5.0, vec3(1.0, 0.0, 0.0), 10.0),
				Cell::new(vec2(30.0, 0.0), 8.0, vec3(0.1, 1.0, 0.2), 0.0),
				Cell::new(vec2(0.0, -40.0), 3.0, vec3(0.2, 0.3, 1.0), 20.0),
			]
		}
	}
}

impl Tick for Simulation {
	fn tick(&mut self, dt: f64) {
		for cell in &mut self.cells {
			cell.tick(dt);
		}
	}
}

impl ObjectProvider<layers::dots::Dot> for Simulation {
	fn iter_objects(&self) -> Box<dyn Iterator<Item = layers::dots::Dot> + '_> {
		let iter = self.cells.iter().map(|cell| layers::dots::Dot {
			coords: cell.coords,
			radius: cell.radius,
			color: cell.color,
			brightness: cell.brightness
		});
		Box::new(iter)
	}
}
