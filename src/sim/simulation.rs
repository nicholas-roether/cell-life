use std::sync::{Arc, Mutex};

use glam::{Vec2, Vec3};

use crate::{
	ecs::{Ecs, Entity},
	particles::{GroupSpawnProps, ParticleSystem},
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
	particle_system: Arc<Mutex<ParticleSystem>>,
	ecs: Mutex<Ecs<Box<dyn Receptor>>>,
	cells: Vec<Mutex<Cell>>
}

struct DeathParticles {
	color: Vec3,
	position: Vec2,
	spread: f32
}

impl Simulation {
	pub fn new(particle_system: Arc<Mutex<ParticleSystem>>) -> Self {
		Self {
			particle_system,
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
		let mut death_particles = Vec::<DeathParticles>::new();
		self.cells.retain(|cell| {
			let cell_lock = cell.lock().unwrap();
			let still_alive = cell_lock.health > 0.0;
			if !still_alive {
				death_particles.push(DeathParticles {
					color: cell_lock.color,
					position: cell_lock.position,
					spread: cell_lock.size
				})
			}
			still_alive
		});
		for particles in death_particles {
			self.spawn_death_particles(particles);
		}
	}

	fn spawn_death_particles(
		&self,
		DeathParticles {
			color,
			spread,
			position
		}: DeathParticles
	) {
		let mut ps_lock = self.particle_system.lock().unwrap();
		ps_lock.spawn_particle_group(GroupSpawnProps {
			color,
			count: 30,
			position,
			velocity: 50.0,
			lifetime: 1.0,
			spread,
			size: 15.0,
			opacity: 0.2
		})
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
