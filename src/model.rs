use crevice::std430::AsStd430;
use glam::{Vec2, Vec3};

#[derive(AsStd430)]
pub struct Dot {
	pub coords: Vec2,
	pub radius: f32,
	pub color: Vec3,
	pub brightness: f32
}
