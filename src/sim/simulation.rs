use std::{
	collections::HashMap,
	sync::{Arc, Mutex, MutexGuard}
};

use glam::{Vec2, Vec3};
use uuid::Uuid;

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
	cells: HashMap<Uuid, Mutex<Cell>>
}

struct DeathParticles {
	color: Vec3,
	position: Vec2,
	spread: f32
}

impl DeathParticles {
	fn for_cell(cell: &MutexGuard<'_, Cell>) -> Self {
		Self {
			color: cell.color,
			position: cell.position,
			spread: cell.size
		}
	}
}

impl Simulation {
	pub fn new(particle_system: Arc<Mutex<ParticleSystem>>) -> Self {
		Self {
			particle_system,
			ecs: Mutex::new(Ecs::new()),
			cells: HashMap::new()
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
		self.cells.insert(Uuid::new_v4(), Mutex::new(cell));
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
		let mut dead_ids = Vec::<Uuid>::new();

		for (id, cell) in &self.cells {
			let cell_lock = cell.lock().unwrap();
			if cell_lock.health > 0.0 {
				continue;
			}
			death_particles.push(DeathParticles::for_cell(&cell_lock));
			dead_ids.push(*id);
		}

		self.kill_cells(&dead_ids);

		for particles in death_particles {
			self.spawn_death_particles(particles);
		}
	}

	fn kill_cells(&mut self, ids: &[Uuid]) {
		for id in ids {
			self.cells.remove(id);
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

	fn get_cells_without(&self, id: Uuid) -> Vec<&Mutex<Cell>> {
		self.cells
			.iter()
			.filter_map(|(other_id, other_cell)| {
				if id == *other_id {
					None
				} else {
					Some(other_cell)
				}
			})
			.collect()
	}
}

impl Tick for Simulation {
	fn tick(&mut self, dt: f64) {
		for (id, cell) in &self.cells {
			let mut cell_lock = cell.lock().unwrap();
			let other_cells = self.get_cells_without(*id);
			cell_lock.tick(&self.ecs, dt, &other_cells);
		}
		self.kill_dead_cells();
	}
}

impl ObjectProvider<layers::dots::Dot> for Simulation {
	fn iter_objects(&self) -> Box<dyn Iterator<Item = layers::dots::Dot> + '_> {
		let iter = self.cells.values().map(|cell| cell.into());
		Box::new(iter)
	}
}
