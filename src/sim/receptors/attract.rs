use std::sync::Mutex;

use glam::{Vec2, Vec3};

use crate::sim::cell::Cell;

use super::{InteractionAccumulator, Receptor};

#[derive(Debug)]
pub struct AttractionReceptor {
	strength: Vec3
}

impl AttractionReceptor {
	pub fn new(strength: Vec3) -> Self {
		Self { strength }
	}
}

struct AttractionAccumulator<'a> {
	receptor: &'a AttractionReceptor,
	force: Vec2
}

impl<'a> AttractionAccumulator<'a> {
	fn new(receptor: &'a AttractionReceptor) -> Self {
		Self {
			receptor,
			force: Vec2::ZERO
		}
	}
}

const ATTRACTION_STRENGTH: f32 = 50.0;
const ATTRACTION_COST: f32 = 0.0000001;
const ATTRACTION_RANGE: f32 = 500.0;

impl<'a> InteractionAccumulator for AttractionAccumulator<'a> {
	fn add_interaction(&mut self, cell: &Cell, other_cell: &Mutex<Cell>) {
		let other_cell_lock = other_cell.lock().unwrap();
		let attraction = self.receptor.strength.dot(other_cell_lock.color);
		let pos_difference = other_cell_lock.position - cell.position;
		let distance = pos_difference.length();
		if distance >= ATTRACTION_RANGE {
			return;
		}

		let force_strength = ATTRACTION_STRENGTH * attraction * other_cell_lock.mass();
		self.force += force_strength * pos_difference.normalize();
	}

	fn complete(&mut self, cell: &mut Cell) -> Vec2 {
		let energy_cost = self.force.length() * ATTRACTION_COST;
		cell.consume_energy(energy_cost) * self.force
	}
}

impl Receptor for AttractionReceptor {
	fn interaction_accumulator<'a>(&'a self) -> Box<dyn InteractionAccumulator + 'a> {
		Box::new(AttractionAccumulator::new(self))
	}
}
