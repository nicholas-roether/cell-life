use std::sync::{Arc, Mutex};

use crevice::std430::{self, AsStd430};
use glam::{vec2, Vec2, Vec3};
use winit::dpi::LogicalSize;

use crate::render::{
	buffer::Buffer, context::GraphicsContext, renderer::Layer, shader::ShaderProgram,
	vertex_model::VertexModel, ObjectProvider
};

#[derive(AsStd430)]
pub struct Dot {
	pub coords: Vec2,
	pub radius: f32,
	pub color: Vec3,
	pub brightness: f32
}

#[allow(unused)]
#[derive(Debug)]
struct Vertex {
	uv: Vec2,
	xy: Vec2
}

pub struct DotsLayer<P: ObjectProvider<Dot>> {
	ctx: GraphicsContext,
	vertex_model: VertexModel,
	vertex_buffer: Buffer,
	obj_buffer: Buffer,
	shader_program: ShaderProgram,
	dot_provider: Arc<Mutex<P>>
}

const VERTEX_SHADER: &str = include_str!("./shaders/dots.vert.glsl");
const FRAGMENT_SHADER: &str = include_str!("./shaders/dots.frag.glsl");

const NUM_VERTICES: usize = 4;

impl<P: ObjectProvider<Dot>> DotsLayer<P> {
	pub fn new(ctx: GraphicsContext, dot_provider: Arc<Mutex<P>>) -> Self {
		let mut vertex_model = ctx.make_vertex_model();
		vertex_model.add_attribute(2, glow::FLOAT);
		vertex_model.add_attribute(2, glow::FLOAT);

		let vertex_buffer = ctx.make_buffer(glow::ARRAY_BUFFER);
		vertex_buffer.bind();
		vertex_model.apply();

		let obj_buffer = ctx.make_buffer(glow::SHADER_STORAGE_BUFFER);

		let shader_program = ctx.make_program(vec![
			ctx.make_shader(glow::VERTEX_SHADER, VERTEX_SHADER),
			ctx.make_shader(glow::FRAGMENT_SHADER, FRAGMENT_SHADER),
		]);

		Self {
			ctx,
			dot_provider,
			vertex_model,
			vertex_buffer,
			obj_buffer,
			shader_program
		}
	}

	fn write_dots(&mut self) {
		let dots: Vec<Dot> = {
			let dot_provider = self
				.dot_provider
				.lock()
				.expect("Failed to get read lock on dot provider");
			dot_provider.iter_objects().collect()
		};

		let mut writer = std430::Writer::new(self.obj_buffer.make_writer(glow::STREAM_DRAW));
		writer
			.write(&(dots.len() as u32))
			.expect("Failed to write to storage buffer");
		writer
			.write(dots.as_slice())
			.expect("Failed to write to storage buffer");
	}

	fn generate_vertices(width: f32, height: f32) -> [Vertex; NUM_VERTICES] {
		[
			Vertex {
				uv: vec2(-1.0, -1.0),
				xy: vec2(-width / 2.0, -height / 2.0)
			},
			Vertex {
				uv: vec2(-1.0, 1.0),
				xy: vec2(-width / 2.0, height / 2.0)
			},
			Vertex {
				uv: vec2(1.0, -1.0),
				xy: vec2(width / 2.0, -height / 2.0)
			},
			Vertex {
				uv: vec2(1.0, 1.0),
				xy: vec2(width / 2.0, height / 2.0)
			}
		]
	}
}

impl<P: ObjectProvider<Dot>> Layer for DotsLayer<P> {
	fn draw(&mut self, size: LogicalSize<f32>) {
		self.vertex_model.bind();
		self.vertex_buffer.bind();
		self.shader_program.activate();

		self.vertex_buffer.set_data(
			&Self::generate_vertices(size.width, size.height),
			glow::STREAM_DRAW
		);

		self.write_dots();
		self.obj_buffer.bind_base(0);

		self.ctx.draw(glow::TRIANGLE_STRIP, NUM_VERTICES);
	}
}
