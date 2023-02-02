use std::sync::Mutex;

use glam::{vec2, vec3};

use crate::render::{layers, ObjectProvider};

use super::{cell::Cell, receptor::AttractionReceptor};

pub trait Tick {
	fn tick(&mut self, dt: f64);
}

#[derive(Debug)]
pub struct Simulation {
	cells: Vec<Mutex<Cell>>
}

impl Simulation {
	pub fn new() -> Self {
		Self {
			cells: vec![
				Mutex::new(Cell::new(
					5.0,
					vec3(1.0, 0.3, 0.3),
					vec2(-30.0, 0.0),
					vec![Box::new(AttractionReceptor::new(vec3(0.0, 1.0, 0.0), 1.0))]
				)),
				Mutex::new(Cell::new(8.0, vec3(0.3, 1.0, 0.3), vec2(30.0, 0.0), vec![])),
				Mutex::new(Cell::new(
					3.0,
					vec3(0.3, 0.3, 1.0),
					vec2(0.0, -40.0),
					vec![]
				)),
			]
		}
	}
}

impl Tick for Simulation {
	fn tick(&mut self, dt: f64) {
		// FIXME this causes a deadlock
		// for (i, cell) in self.cells.iter().enumerate() {
		// 	let mut cell_lock = cell.lock().unwrap();
		// 	let other_cells: Vec<&Mutex<Cell>> = self
		// 		.cells
		// 		.iter()
		// 		.enumerate()
		// 		.filter_map(|(j, cell)| {
		// 			if i == j {
		// 				return None;
		// 			}
		// 			Some(cell)
		// 		})
		// 		.collect();
		// 	cell_lock.tick(dt, Box::new(other_cells.into_iter()));
		// }
	}
}

impl ObjectProvider<layers::dots::Dot> for Simulation {
	fn iter_objects(&self) -> Box<dyn Iterator<Item = layers::dots::Dot> + '_> {
		let iter = self
			.cells
			.iter()
			.map(|cell| (&cell.lock().unwrap().state).into());
		Box::new(iter)
	}
}
