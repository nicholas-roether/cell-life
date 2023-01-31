use std::{mem::size_of, rc::Rc};

use crevice::std430::{self, AsStd430, Vec2, Vec3};
use glow::HasContext;
use winit::dpi::LogicalSize;

use crate::window;

use super::{
	buffer::Buffer,
	shader::{Shader, ShaderProgram},
	vertex_model::VertexModel
};

#[derive(AsStd430)]
pub struct Dot {
	pub coords: Vec2,
	pub radius: f32,
	pub color: Vec3,
	pub brightness: f32
}

pub struct Renderer {
	gl: Rc<glow::Context>,
	vertex_buffer: Buffer,
	_obj_buffer: Buffer,
	_shader_program: ShaderProgram
}

#[allow(unused)]
#[derive(Debug)]
struct Vertex {
	u: f32,
	v: f32,
	x: f32,
	y: f32
}

const VERTEX_SHADER: &str = include_str!("../shaders/test.vert.glsl");
const FRAGMENT_SHADER: &str = include_str!("../shaders/test.frag.glsl");

const NUM_VERTICES: usize = 4;

macro_rules! to_binary {
	($slice:expr, $type:ty) => {
		::std::slice::from_raw_parts(
			(&$slice as *const _) as *const u8,
			$slice.len() * ::std::mem::size_of::<$type>()
		)
	};
}

impl window::Renderer for Renderer {
	fn draw(&mut self, LogicalSize { width, height }: LogicalSize<f32>) {
		self.vertex_buffer
			.set_data(&Self::vertex_data(width, height), glow::STREAM_DRAW);

		unsafe {
			self.gl.clear(glow::COLOR_BUFFER_BIT);
			self.gl
				.draw_arrays(glow::TRIANGLE_STRIP, 0, NUM_VERTICES as i32);
		}
	}
}

impl Renderer {
	pub fn new(gl: Rc<glow::Context>, dots: Vec<Dot>) -> Self {
		Self::bind_vertex_array(&gl);

		let vertex_buffer = Buffer::new(Rc::clone(&gl), glow::ARRAY_BUFFER);

		let mut vertex_model = VertexModel::new(Rc::clone(&gl));
		vertex_model.add_attribute(2, glow::FLOAT);
		vertex_model.add_attribute(2, glow::FLOAT);
		vertex_model.apply();

		let mut obj_buffer = Buffer::new(Rc::clone(&gl), glow::SHADER_STORAGE_BUFFER);

		let mut writer = std430::Writer::new(obj_buffer.make_writer(glow::STATIC_DRAW));
		writer.write(&(dots.len() as u32)).unwrap();
		writer.write(dots.as_slice()).unwrap();

		let vertex_shader = Shader::new(Rc::clone(&gl), glow::VERTEX_SHADER, VERTEX_SHADER);
		let fragment_shader = Shader::new(Rc::clone(&gl), glow::FRAGMENT_SHADER, FRAGMENT_SHADER);
		let shader_program =
			ShaderProgram::new(Rc::clone(&gl), vec![vertex_shader, fragment_shader]);
		shader_program.activate();

		obj_buffer.bind_base(0);

		Self {
			gl,
			vertex_buffer,
			_obj_buffer: obj_buffer,
			_shader_program: shader_program
		}
	}

	fn bind_vertex_array(gl: &glow::Context) {
		unsafe {
			let vertex_array = gl
				.create_vertex_array()
				.expect("Failed to create vertex array");
			gl.bind_vertex_array(Some(vertex_array));
		}
	}

	fn vertex_data(width: f32, height: f32) -> [u8; size_of::<Vertex>() * NUM_VERTICES] {
		let vertices = Self::generate_vertices(width, height);
		let slice = unsafe { to_binary!(vertices, Vertex) };
		slice.try_into().unwrap()
	}

	fn generate_vertices(width: f32, height: f32) -> [Vertex; NUM_VERTICES] {
		[
			Vertex {
				u: -1.0,
				v: -1.0,
				x: -width / 2.0,
				y: -height / 2.0
			},
			Vertex {
				u: -1.0,
				v: 1.0,
				x: -width / 2.0,
				y: height / 2.0
			},
			Vertex {
				u: 1.0,
				v: -1.0,
				x: width / 2.0,
				y: -height / 2.0
			},
			Vertex {
				u: 1.0,
				v: 1.0,
				x: width / 2.0,
				y: height / 2.0
			}
		]
	}
}
