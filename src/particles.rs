use std::{f32::consts::TAU, time::SystemTime};

use glam::{Vec2, Vec3};
use rand::random;

use crate::render::{layers, ObjectProvider};

#[derive(Debug)]
struct ParticleState {
	position: Vec2,
	rotation: f32
}

#[derive(Debug)]
struct Particle {
	shape: Vec<Vec2>,
	velocity: Vec2,
	angular_velocity: f32,
	lifetime: f32,
	opacity: f32,
	birthtime: SystemTime,
	state: ParticleState
}

#[derive(Debug)]
struct ParticleGroup {
	color: Vec3,
	particles: Vec<Particle>
}

#[derive(Debug)]
pub struct ParticleSystem {
	groups: Vec<ParticleGroup>
}

pub struct GroupSpawnProps {
	pub color: Vec3,
	pub count: usize,
	pub position: Vec2,
	pub velocity: f32,
	pub lifetime: f32,
	pub spread: f32,
	pub size: f32,
	pub opacity: f32
}

const OPACITY_SPREAD: f32 = 0.15;
const MAX_ANGULAR_VELOCITY: f32 = 1.0;
const LIFETIME_SPREAD: f32 = 0.3;

impl ParticleSystem {
	pub fn new() -> Self {
		Self { groups: Vec::new() }
	}

	pub fn spawn_particle_group(
		&mut self,
		GroupSpawnProps {
			color,
			count,
			position,
			velocity,
			lifetime,
			spread,
			size,
			opacity
		}: GroupSpawnProps
	) {
		let offs_positions = Self::generate_points_in_radius(spread, count);
		let particles: Vec<Particle> = offs_positions
			.into_iter()
			.map(|offs_pos| {
				let vel = velocity * offs_pos / spread;
				Self::generate_particle(position + offs_pos, vel, lifetime, size, opacity)
			})
			.collect();
		let group = ParticleGroup { color, particles };
		self.groups.push(group);
	}

	pub fn tick(&mut self, dt: f64) {
		for group in &mut self.groups {
			for particle in &mut group.particles {
				particle.state.position += particle.velocity * dt as f32;
				particle.state.rotation += particle.angular_velocity * dt as f32;
			}
			Self::remove_dead_particles(group)
		}
	}

	fn remove_dead_particles(group: &mut ParticleGroup) {
		group
			.particles
			.retain(|p| p.birthtime.elapsed().unwrap().as_secs_f32() < p.lifetime);
	}

	fn generate_point_in_radius(radius: f32) -> Vec2 {
		let rand_radius = random::<f32>().sqrt() * radius;
		let rand_angle = TAU * random::<f32>();
		rand_radius * Vec2::from_angle(rand_angle)
	}

	fn generate_points_in_radius(radius: f32, num_points: usize) -> Vec<Vec2> {
		let mut points = Vec::with_capacity(num_points);
		for _ in 0..num_points {
			points.push(Self::generate_point_in_radius(radius))
		}
		points
	}

	fn generate_opacity(opacity: f32) -> f32 {
		opacity + OPACITY_SPREAD * (random::<f32>() - 1.0) / 2.0
	}

	fn generate_lifetime(base_lifetime: f32) -> f32 {
		base_lifetime + LIFETIME_SPREAD * (random::<f32>() - 1.0) / 2.0
	}

	fn generate_angular_velocity() -> f32 {
		MAX_ANGULAR_VELOCITY * random::<f32>()
	}

	fn generate_particle(
		position: Vec2,
		velocity: Vec2,
		base_lifetime: f32,
		size: f32,
		opacity: f32
	) -> Particle {
		Particle {
			shape: Self::generate_points_in_radius(size, 3),
			velocity,
			angular_velocity: Self::generate_angular_velocity(),
			lifetime: Self::generate_lifetime(base_lifetime),
			birthtime: SystemTime::now(),
			opacity: Self::generate_opacity(opacity),
			state: ParticleState {
				position,
				rotation: 0.0
			}
		}
	}
}

impl ObjectProvider<layers::particles::ParticleGroup> for ParticleSystem {
	fn iter_objects(&self) -> Box<dyn Iterator<Item = layers::particles::ParticleGroup> + '_> {
		Box::new(self.groups.iter().map(|group| {
			layers::particles::ParticleGroup::new(
				group.color,
				group
					.particles
					.iter()
					.map(|particle| {
						let time_since_birth = particle.birthtime.elapsed().unwrap().as_secs_f32();
						layers::particles::Particle::new(
							particle.opacity * f32::max(particle.lifetime - time_since_birth, 0.0),
							particle.state.position,
							particle.state.rotation,
							particle.shape.clone()
						)
					})
					.collect()
			)
		}))
	}
}
