use std::{fmt::Debug, sync::Mutex};

use glam::{Vec2, Vec3};

use super::cell::Cell;

pub trait InteractionAccumulator {
	fn add_interaction(&mut self, cell: &Cell, other_cell: &Mutex<Cell>);

	fn complete(&mut self, cell: &mut Cell) -> Vec2;
}

pub trait Receptor: Debug + Send + Sync {
	fn interaction_accumulator<'a>(&'a self) -> Box<dyn InteractionAccumulator + 'a>;
}

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
