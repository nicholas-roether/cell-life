use std::sync::Mutex;

use glam::Vec2;

use crate::sim::cell::Cell;

use super::{InteractionAccumulator, Receptor};

#[derive(Debug)]
pub struct BaseReceptor;

impl BaseReceptor {
	#[inline]
	pub fn new() -> Self {
		Self {}
	}
}

pub struct BaseAccumulator {
	force: Vec2
}

const BASE_REPULSION_STRENGTH: f32 = 3000000.0;
const FRICTION: f32 = 10.0;

impl BaseAccumulator {
	fn new() -> Self {
		Self { force: Vec2::ZERO }
	}
}

impl InteractionAccumulator for BaseAccumulator {
	fn add_interaction(&mut self, cell: &Cell, other_cell: &Mutex<Cell>) {
		let pos_difference = {
			let other_cell_lock = other_cell.lock().unwrap();
			other_cell_lock.position - cell.position
		};
		let distance = pos_difference.length();
		let direction = pos_difference.normalize();

		let force_strength = BASE_REPULSION_STRENGTH * (cell.size / distance).powi(2);
		self.force -= force_strength * direction;
	}

	fn complete(&mut self, cell: &mut Cell) -> Vec2 {
		self.force - FRICTION * cell.velocity * cell.mass()
	}
}

impl Receptor for BaseReceptor {
	fn interaction_accumulator<'a>(&'a self) -> Box<dyn InteractionAccumulator + 'a> {
		Box::new(BaseAccumulator::new())
	}
}
