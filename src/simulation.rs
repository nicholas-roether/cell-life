use crevice::std430::AsStd430;
use glam::{Vec2, Vec3};

use crate::render::layers::dots::DotProvider;

#[derive(AsStd430)]
pub struct Dot {
	pub coords: Vec2,
	pub radius: f32,
	pub color: Vec3,
	pub brightness: f32
}

pub trait Tick {
	fn tick(&mut self, time: f32);
}

pub struct Simulation {
	pub dots: Vec<Dot>
}

impl Tick for Simulation {
	fn tick(&mut self, time: f32) {
		for dot in &mut self.dots {
			dot.tick(time);
		}
	}
}

impl Tick for Dot {
	fn tick(&mut self, _time: f32) {
		todo!()
	}
}

impl DotProvider for Simulation {
	fn get_dots(&self) -> &'_ [Dot] {
		&self.dots
	}
}
