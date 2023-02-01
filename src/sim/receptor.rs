use glam::Vec3;

use super::cell::Cell;

pub trait Receptor {
	const COLOR: Vec3;

	fn apply_effect<'a, I: Iterator<Item = &'a mut Cell>>(
		&self,
		cell: &'a mut Cell,
		other_cells: I
	) -> f32;
}
