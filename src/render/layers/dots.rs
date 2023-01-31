use std::rc::Rc;

use crevice::std430;
use glam::{vec2, Vec2};
use glow::HasContext;
use winit::dpi::LogicalSize;

use crate::{
	model::Dot,
	render::{
		buffer::Buffer,
		renderer::Layer,
		shader::{Shader, ShaderProgram},
		vertex_model::VertexModel
	}
};

#[allow(unused)]
#[derive(Debug)]
struct Vertex {
	uv: Vec2,
	xy: Vec2
}

pub struct DotsLayer {
	gl: Rc<glow::Context>,
	vertex_model: VertexModel,
	vertex_buffer: Buffer,
	obj_buffer: Buffer,
	shader_program: ShaderProgram
}

const VERTEX_SHADER: &str = include_str!("./shaders/dots.vert.glsl");
const FRAGMENT_SHADER: &str = include_str!("./shaders/dots.frag.glsl");

const NUM_VERTICES: usize = 4;

impl DotsLayer {
	pub fn new(gl: Rc<glow::Context>, dots: Vec<Dot>) -> Self {
		let mut vertex_model = VertexModel::new(Rc::clone(&gl));
		vertex_model.add_attribute(2, glow::FLOAT);
		vertex_model.add_attribute(2, glow::FLOAT);

		let vertex_buffer = Buffer::new(Rc::clone(&gl), glow::ARRAY_BUFFER);
		vertex_buffer.bind();
		vertex_model.apply();

		let mut obj_buffer = Buffer::new(Rc::clone(&gl), glow::SHADER_STORAGE_BUFFER);
		let mut writer = std430::Writer::new(obj_buffer.make_writer(glow::STATIC_DRAW));
		writer
			.write(&(dots.len() as u32))
			.expect("Failed to write to storage buffer");
		writer
			.write(dots.as_slice())
			.expect("Failed to write to storage buffer");

		let vertex_shader = Shader::new(Rc::clone(&gl), glow::VERTEX_SHADER, VERTEX_SHADER);
		let fragment_shader = Shader::new(Rc::clone(&gl), glow::FRAGMENT_SHADER, FRAGMENT_SHADER);
		let shader_program =
			ShaderProgram::new(Rc::clone(&gl), vec![vertex_shader, fragment_shader]);

		shader_program.activate();
		vertex_model.bind();
		obj_buffer.bind_base(0);

		Self {
			gl,
			vertex_model,
			vertex_buffer,
			obj_buffer,
			shader_program
		}
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

impl Layer for DotsLayer {
	fn draw(&mut self, size: LogicalSize<f32>) {
		self.vertex_model.bind();
		self.vertex_buffer.bind();
		self.obj_buffer.bind();
		self.shader_program.activate();

		self.vertex_buffer.set_data(
			&Self::generate_vertices(size.width, size.height),
			glow::STREAM_DRAW
		);

		unsafe {
			self.gl
				.draw_arrays(glow::TRIANGLE_STRIP, 0, NUM_VERTICES as i32);
		}
	}
}
