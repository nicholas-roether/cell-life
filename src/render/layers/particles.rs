use std::sync::{Arc, Mutex};

use glam::{vec2, Vec2, Vec3};
use winit::dpi::LogicalSize;

use crate::render::{
	buffer::Buffer, renderer::Layer, shader::ShaderProgram, uniform::UniformVec3,
	vertex_model::VertexModel, GraphicsContext, ObjectProvider
};

#[derive(Debug)]
pub struct Particle {
	opacity: f32,
	position: Vec2,
	rotation: f32,
	shape: Vec<Vec2>
}

impl Particle {
	pub fn new(opacity: f32, position: Vec2, rotation: f32, shape: Vec<Vec2>) -> Self {
		Self {
			opacity,
			position,
			rotation,
			shape
		}
	}
}

#[derive(Debug)]
pub struct ParticleGroup {
	color: Vec3,
	particles: Vec<Particle>
}

impl ParticleGroup {
	pub fn new(color: Vec3, particles: Vec<Particle>) -> Self {
		Self { color, particles }
	}
}

#[allow(unused)]
#[derive(Debug)]
struct Vertex {
	center: Vec2,
	offs_position: Vec2,
	rotation: f32,
	opacity: f32
}

pub struct ParticlesLayer<P: ObjectProvider<ParticleGroup>> {
	ctx: GraphicsContext,
	vertex_model: VertexModel,
	vertex_buffer: Buffer,
	shader_program: ShaderProgram,
	color_uniform: UniformVec3,
	particle_group_provider: Arc<Mutex<P>>
}

const VERTEX_SHADER: &str = include_str!("./shaders/particles.vert.glsl");
const FRAGMENT_SHADER: &str = include_str!("./shaders/particles.frag.glsl");

impl<P: ObjectProvider<ParticleGroup>> ParticlesLayer<P> {
	pub fn new(ctx: GraphicsContext, particle_group_provider: Arc<Mutex<P>>) -> Self {
		let mut vertex_model = ctx.make_vertex_model();
		vertex_model.add_attribute(2, glow::FLOAT);
		vertex_model.add_attribute(2, glow::FLOAT);
		vertex_model.add_attribute(1, glow::FLOAT);
		vertex_model.add_attribute(1, glow::FLOAT);

		let vertex_buffer = ctx.make_buffer(glow::ARRAY_BUFFER);
		vertex_buffer.bind();
		vertex_model.apply();

		let shader_program = ctx.make_program(vec![
			ctx.make_shader(glow::VERTEX_SHADER, VERTEX_SHADER),
			ctx.make_shader(glow::FRAGMENT_SHADER, FRAGMENT_SHADER),
		]);

		shader_program.activate();
		let color_uniform = shader_program.get_uniform_vec3("color");

		Self {
			ctx,
			vertex_model,
			vertex_buffer,
			shader_program,
			color_uniform,
			particle_group_provider
		}
	}

	fn write_vertices(&mut self, group: ParticleGroup, size: LogicalSize<f32>) -> usize {
		let mut vertices = Vec::<Vertex>::new();
		for particle in group.particles {
			for point in particle.shape {
				vertices.push(Vertex {
					center: 2.0 * particle.position / vec2(size.width, size.height),
					rotation: particle.rotation,
					offs_position: 2.0 * point / vec2(size.width, size.height),
					opacity: particle.opacity
				});
			}
		}
		self.vertex_buffer.set_data(&vertices, glow::STREAM_DRAW);
		vertices.len()
	}

	fn draw_group(&mut self, group: ParticleGroup, size: LogicalSize<f32>) {
		self.color_uniform.set(group.color);
		let num_vertices = self.write_vertices(group, size);
		self.ctx.draw(glow::TRIANGLES, num_vertices);
	}

	fn draw_groups(&mut self, size: LogicalSize<f32>) {
		let particle_groups: Vec<ParticleGroup> = {
			let group_provider_lock = self.particle_group_provider.lock().unwrap();
			group_provider_lock.iter_objects().collect()
		};

		for group in particle_groups {
			self.draw_group(group, size);
		}
	}
}

impl<P: ObjectProvider<ParticleGroup>> Layer for ParticlesLayer<P> {
	fn draw(&mut self, size: LogicalSize<f32>) {
		self.vertex_model.bind();
		self.vertex_buffer.bind();
		self.shader_program.activate();

		self.draw_groups(size);
	}
}
