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
	pub health: f32,
	pub size: f32,
	pub color: Vec3,
	pub energy: f64,
	pub position: Vec2,
	pub velocity: Vec2,
	pub acceleration: Vec2
}

const DENSITY: f32 = 1.0;
const REGEN_SPEED: f32 = 0.2;
const MAX_HEALTH: f32 = 3.0;

impl Cell {
	pub fn consume_energy(&mut self, cost: f64) -> f32 {
		if self.energy <= 0.0 {
			return 0.0;
		}
		if self.energy >= cost {
			self.energy -= cost;
			return 1.0;
		}
		let fraction_available = self.energy / cost;
		self.energy = 0.0;
		fraction_available as f32
	}

	pub fn mass(&self) -> f32 {
		PI * self.size.powi(2) * DENSITY
	}

	fn handle_health(&mut self, dt: f32) {
		if self.energy == 0.0 {
			self.health -= dt;
		} else if self.health <= MAX_HEALTH {
			self.health = f32::min(self.health + REGEN_SPEED * dt, MAX_HEALTH)
		}
	}

	fn apply_force(&mut self, force: Vec2) {
		self.acceleration = force / self.mass();
	}

	fn sim_movement(&mut self, dt: f32) {
		self.velocity += self.acceleration * dt;
		self.position += self.velocity * dt;
	}

	fn apply_effects(
		&mut self,
		ecs: &Mutex<Ecs<Box<dyn Receptor>>>,
		other_cells: &[&Mutex<Cell>],
		dt: f64
	) {
		self.acceleration = Vec2::ZERO;
		self.apply_receptor_effects(ecs, other_cells, dt);
	}

	fn apply_receptor_effects(
		&mut self,
		ecs: &Mutex<Ecs<Box<dyn Receptor>>>,
		other_cells: &[&Mutex<Cell>],
		dt: f64
	) {
		let ecs_lock = ecs.lock().unwrap();
		let mut accumulators: Vec<Box<dyn InteractionAccumulator>> = ecs_lock
			.components(self.entity)
			.iter()
			.map(|rec| rec.interaction_accumulator())
			.collect();

		for other_cell in other_cells {
			for acc in &mut accumulators {
				acc.add_interaction(self, other_cell, dt)
			}
		}

		let mut force = Vec2::ZERO;

		for acc in &mut accumulators {
			force += acc.complete(self, dt);
		}

		self.apply_force(force);
	}
}

impl Cell {
	pub fn new(entity: Entity) -> Self {
		Self {
			entity,
			health: 3.0,
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
		self.handle_health(dt as f32);
		self.apply_effects(ecs, other_cells, dt);
	}
}

impl From<&Mutex<Cell>> for layers::dots::Dot {
	fn from(value: &Mutex<Cell>) -> Self {
		let state_lock = value.lock().unwrap();
		layers::dots::Dot {
			coords: state_lock.position,
			radius: state_lock.size,
			color: state_lock.color,
			brightness: state_lock.energy as f32
		}
	}
}
