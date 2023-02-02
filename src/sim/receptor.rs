use std::{fmt::Debug, sync::Mutex};

use glam::{Vec2, Vec3};

use crate::utils::Accumulator;

use super::cell::{Cell, CellState};

pub trait Receptor: Debug + Send + Sync {
	fn apply_effect<'a>(
		&'a self,
		cell_state: &'a Mutex<CellState>
	) -> Box<dyn Accumulator<&'a Mutex<Cell>, ()> + 'a>;
}

#[derive(Debug)]
pub struct AttractionReceptor {
	color: Vec3,
	strength: f32
}

impl AttractionReceptor {
	pub fn new(color: Vec3, strength: f32) -> Self {
		Self { color, strength }
	}
}

struct AttractionAccumulator<'a> {
	receptor: &'a AttractionReceptor,
	cell_state: &'a Mutex<CellState>,
	force: Vec2
}

impl<'a> AttractionAccumulator<'a> {
	fn new(receptor: &'a AttractionReceptor, cell_state: &'a Mutex<CellState>) -> Self {
		Self {
			receptor,
			cell_state,
			force: Vec2::ZERO
		}
	}
}

impl<'a> Accumulator<&Mutex<Cell>, ()> for AttractionAccumulator<'a> {
	fn accumulate(&mut self, other_cell: &Mutex<Cell>) {
		let state_lock = self.cell_state.lock().unwrap();
		let other_cell_lock = other_cell.lock().unwrap();
		let other_cell_state_lock = other_cell_lock.state.lock().unwrap();
		let attraction = self.receptor.color.dot(other_cell_state_lock.color);
		let pos_difference = other_cell_state_lock.position - state_lock.position;
		let distance = pos_difference.length();
		let force_strength =
			attraction * self.receptor.strength * other_cell_state_lock.size / distance;
		self.force += force_strength * pos_difference.normalize();
	}

	fn complete(&mut self) -> () {
		let mut state_lock = self.cell_state.lock().unwrap();
		let energy_cost = self.force.length();
		let full_acceleration = self.force / state_lock.size;
		let actual_acceleration = state_lock.consume_energy(energy_cost) * full_acceleration;
		state_lock.acceleration += actual_acceleration;
	}
}

impl Receptor for AttractionReceptor {
	fn apply_effect<'a>(
		&'a self,
		cell_state: &'a Mutex<CellState>
	) -> Box<dyn Accumulator<&'a Mutex<Cell>, ()> + 'a> {
		Box::new(AttractionAccumulator::new(self, cell_state))
	}
}
