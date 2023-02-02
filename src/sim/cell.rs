use std::sync::Mutex;

use glam::{Vec2, Vec3};

use crate::{render::layers, utils::Accumulator};

use super::receptor::Receptor;

#[derive(Debug)]
pub struct CellState {
	pub size: f32,
	pub color: Vec3,
	pub energy: f32,
	pub position: Vec2,
	pub velocity: Vec2,
	pub acceleration: Vec2
}

impl CellState {
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
}

#[derive(Debug)]
pub struct Cell {
	pub state: Mutex<CellState>,
	receptors: Vec<Box<dyn Receptor>>
}

impl Cell {
	pub fn new(size: f32, color: Vec3, position: Vec2, receptors: Vec<Box<dyn Receptor>>) -> Self {
		Self {
			state: Mutex::new(CellState {
				size,
				color,
				energy: 0.0,
				position,
				velocity: Vec2::ZERO,
				acceleration: Vec2::ZERO
			}),
			receptors
		}
	}

	pub fn tick(&mut self, dt: f64, other_cells: Box<dyn Iterator<Item = &Mutex<Cell>> + '_>) {
		let mut state_lock = self.state.lock().unwrap();

		let delta_velocity = state_lock.acceleration * dt as f32;
		state_lock.velocity += delta_velocity;

		let delta_position = state_lock.velocity * dt as f32;
		state_lock.position += delta_position;

		let mut accumulators: Vec<Box<dyn Accumulator<&Mutex<Cell>, ()>>> = vec![];

		for rec in &self.receptors {
			accumulators.push(rec.apply_effect(&self.state));
		}

		for cell in other_cells {
			for acc in &mut accumulators {
				acc.accumulate(&cell)
			}
		}

		for acc in &mut accumulators {
			acc.complete();
		}
	}
}

impl From<&Mutex<CellState>> for layers::dots::Dot {
	fn from(value: &Mutex<CellState>) -> Self {
		let state_lock = value.lock().unwrap();
		layers::dots::Dot {
			coords: state_lock.position,
			radius: state_lock.size,
			color: state_lock.color,
			brightness: state_lock.energy
		}
	}
}
