use std::{f32::consts::PI, sync::Mutex};

use glam::{Vec2, Vec3};

use crate::{
	ecs::{Ecs, Entity},
	render::layers
};

use super::receptors::{InteractionAccumulator, Receptor};

#[derive(Debug)]
pub struct Cell {
	pub entity: Entity,
	pub size: f32,
	pub color: Vec3,
	pub energy: f32,
	pub position: Vec2,
	pub velocity: Vec2,
	pub acceleration: Vec2
}

const DENSITY: f32 = 1.0;

impl Cell {
	pub fn consume_energy(&mut self, cost: f32) -> f32 {
		if self.energy <= 0.0 {
			return 0.0;
		}
		if self.energy >= cost {
			self.energy -= cost;
			return 1.0;
		}
		let fraction_available = self.energy / cost;
		self.energy = 0.0;
		fraction_available
	}

	pub fn mass(&self) -> f32 {
		PI * self.size.powi(2) * DENSITY
	}

	fn apply_force(&mut self, force: Vec2) {
		self.acceleration = force / self.mass();
	}

	fn sim_movement(&mut self, dt: f32) {
		self.velocity += self.acceleration * dt;
		self.position += self.velocity * dt;
	}

	fn apply_effects(&mut self, ecs: &Mutex<Ecs<Box<dyn Receptor>>>, other_cells: &[&Mutex<Cell>]) {
		self.acceleration = Vec2::ZERO;
		self.apply_receptor_effects(ecs, other_cells);
	}

	fn apply_receptor_effects(
		&mut self,
		ecs: &Mutex<Ecs<Box<dyn Receptor>>>,
		other_cells: &[&Mutex<Cell>]
	) {
		let ecs_lock = ecs.lock().unwrap();
		let mut accumulators: Vec<Box<dyn InteractionAccumulator>> = ecs_lock
			.components(self.entity)
			.iter()
			.map(|rec| rec.interaction_accumulator())
			.collect();

		for other_cell in other_cells {
			for acc in &mut accumulators {
				acc.add_interaction(self, other_cell)
			}
		}

		let mut force = Vec2::ZERO;

		for acc in &mut accumulators {
			force += acc.complete(self);
		}

		self.apply_force(force);
	}
}

impl Cell {
	pub fn new(entity: Entity) -> Self {
		Self {
			entity,
			size: 0.0,
			color: Vec3::ZERO,
			energy: 10.0,
			position: Vec2::ZERO,
			velocity: Vec2::ZERO,
			acceleration: Vec2::ZERO
		}
	}

	pub fn tick(
		&mut self,
		ecs: &Mutex<Ecs<Box<dyn Receptor>>>,
		dt: f64,
		other_cells: &[&Mutex<Cell>]
	) {
		self.sim_movement(dt as f32);
		self.apply_effects(ecs, other_cells);
	}
}

impl From<&Mutex<Cell>> for layers::dots::Dot {
	fn from(value: &Mutex<Cell>) -> Self {
		let state_lock = value.lock().unwrap();
		layers::dots::Dot {
			coords: state_lock.position,
			radius: state_lock.size,
			color: state_lock.color,
			brightness: state_lock.energy
		}
	}
}
