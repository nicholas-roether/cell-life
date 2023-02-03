use std::{fmt::Debug, sync::Mutex};

use glam::Vec2;

use super::cell::Cell;

pub mod attract;
pub mod base;

pub trait InteractionAccumulator {
	fn add_interaction(&mut self, cell: &Cell, other_cell: &Mutex<Cell>);

	fn complete(&mut self, cell: &mut Cell) -> Vec2;
}

pub trait Receptor: Debug + Send + Sync {
	fn interaction_accumulator<'a>(&'a self) -> Box<dyn InteractionAccumulator + 'a>;
}
