use std::sync::Mutex;

use glam::{Vec2, Vec3};

use crate::{
	ecs::{Ecs, Entity},
	render::{layers, ObjectProvider}
};

use super::{
	cell::Cell,
	receptors::{base::BaseReceptor, Receptor}
};

pub trait Tick {
	fn tick(&mut self, dt: f64);
}

#[derive(Debug)]
pub struct Simulation {
	ecs: Mutex<Ecs<Box<dyn Receptor>>>,
	cells: Vec<Mutex<Cell>>
}

impl Simulation {
	pub fn new() -> Self {
		Self {
			ecs: Mutex::new(Ecs::new()),
			cells: Vec::new()
		}
	}

	pub fn add_cell(
		&mut self,
		size: f32,
		color: Vec3,
		position: Vec2,
		receptors: Vec<Box<dyn Receptor>>
	) {
		let entity = self.create_cell_entity(receptors);
		let mut cell = Cell::new(entity);
		cell.size = size;
		cell.color = color;
		cell.position = position;
		self.cells.push(Mutex::new(cell));
	}

	fn create_cell_entity(&mut self, receptors: Vec<Box<dyn Receptor>>) -> Entity {
		let mut ecs_lock = self.ecs.lock().unwrap();
		let entity = ecs_lock.entity();
		ecs_lock.add_component(entity, Box::new(BaseReceptor::new()));
		for receptor in receptors {
			ecs_lock.add_component(entity, receptor);
		}
		entity
	}

	fn kill_dead_cells(&mut self) {
		self.cells.retain(|cell| {
			let cell_lock = cell.lock().unwrap();
			cell_lock.health > 0.0
		});
	}
}

impl Tick for Simulation {
	fn tick(&mut self, dt: f64) {
		for (i, cell) in self.cells.iter().enumerate() {
			let mut cell_lock = cell.lock().unwrap();
			let other_cells: Vec<&Mutex<Cell>> = self
				.cells
				.iter()
				.enumerate()
				.filter_map(|(j, cell)| if i == j { None } else { Some(cell) })
				.collect();
			cell_lock.tick(&self.ecs, dt, &other_cells);
		}
		self.kill_dead_cells();
	}
}

impl ObjectProvider<layers::dots::Dot> for Simulation {
	fn iter_objects(&self) -> Box<dyn Iterator<Item = layers::dots::Dot> + '_> {
		let iter = self.cells.iter().map(|cell| cell.into());
		Box::new(iter)
	}
}
