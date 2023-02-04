use std::f32::consts::TAU;

use glam::Vec2;
use rand::random;

#[inline]
pub fn rand() -> f32 {
	random::<f32>()
}

#[inline]
pub fn rand_in_range(min: f32, max: f32) -> f32 {
	min + random::<f32>() * (max - min)
}

pub fn rand_with_spread(avg: f32, spread: f32) -> f32 {
	let half_spread = spread / 2.0;
	rand_in_range(avg - half_spread, avg + half_spread)
}

pub fn rand_point_in_circle(radius: f32) -> Vec2 {
	let abs = (rand() * radius).sqrt();
	let angle = rand() * TAU;
	abs * Vec2::from_angle(angle)
}
