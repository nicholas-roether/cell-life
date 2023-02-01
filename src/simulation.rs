use glam::{Vec2, Vec3};

use crate::render::{layers, ObjectProvider};

pub trait Tick {
	fn tick(&mut self, time: f32);
}

pub struct Cell {
	pub coords: Vec2,
	pub radius: f32,
	pub color: Vec3,
	pub brightness: f32
}

impl Tick for Cell {
	fn tick(&mut self, _time: f32) {
		todo!()
	}
}

pub struct Simulation {
	pub cells: Vec<Cell>
}

impl Tick for Simulation {
	fn tick(&mut self, time: f32) {
		for cell in &mut self.cells {
			cell.tick(time);
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
