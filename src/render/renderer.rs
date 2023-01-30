use std::{mem::size_of, rc::Rc};

use crevice::std430::{self, AsStd430, Vec2, Vec3};
use glow::HasContext;
use winit::dpi::LogicalSize;

use crate::window;

use super::{array_buffer::ArrayBuffer, buffer::Buffer};

#[derive(AsStd430)]
pub struct Dot {
	pub coords: Vec2,
	pub radius: f32,
	pub color: Vec3,
	pub brightness: f32
}

pub struct Renderer {
	gl: Option<Rc<glow::Context>>,
	dots: Vec<Dot>
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
	fn init(&mut self, gl: Rc<glow::Context>) {
		self.gl = Some(Rc::clone(&gl));
		self.bind_vertex_array();

		let mut vertex_buffer = ArrayBuffer::new(Rc::clone(&gl));
		vertex_buffer.add_attribute(2, glow::FLOAT);
		vertex_buffer.add_attribute(2, glow::FLOAT);
		vertex_buffer.bind();

		let obj_buffer = Buffer::new(Rc::clone(&gl), glow::SHADER_STORAGE_BUFFER);
		let mut writer = std430::Writer::new(obj_buffer.make_writer(glow::STATIC_DRAW));
		writer.write(&(self.dots.len() as u32)).unwrap();
		writer.write(self.dots.as_slice()).unwrap();
		obj_buffer.bind();

		unsafe {
			let vertex_shader = gl
				.create_shader(glow::VERTEX_SHADER)
				.expect("Failed to create vertex shader");
			gl.shader_source(vertex_shader, VERTEX_SHADER);
			gl.compile_shader(vertex_shader);
			assert!(gl.get_shader_compile_status(vertex_shader));

			let fragment_shader = gl
				.create_shader(glow::FRAGMENT_SHADER)
				.expect("Failed to create fragment shader");
			gl.shader_source(fragment_shader, FRAGMENT_SHADER);
			gl.compile_shader(fragment_shader);
			assert!(gl.get_shader_compile_status(fragment_shader));

			let shader_program = gl
				.create_program()
				.expect("Failed to create shader program");
			gl.attach_shader(shader_program, vertex_shader);
			gl.attach_shader(shader_program, fragment_shader);
			gl.link_program(shader_program);
			assert!(gl.get_program_link_status(shader_program));
			gl.delete_shader(vertex_shader);
			gl.delete_shader(fragment_shader);
			gl.use_program(Some(shader_program));
		}
	}

	fn draw(&mut self, LogicalSize { width, height }: LogicalSize<f32>) {
		let gl = self.gl.as_ref().unwrap();
		unsafe {
			gl.buffer_data_u8_slice(
				glow::ARRAY_BUFFER,
				&Self::vertex_data(width, height),
				glow::STATIC_DRAW
			);
			gl.clear(glow::COLOR_BUFFER_BIT);
			gl.draw_arrays(glow::TRIANGLE_STRIP, 0, NUM_VERTICES as i32);
		}
	}
}

impl Renderer {
	pub fn new(dots: Vec<Dot>) -> Self {
		Self { dots, gl: None }
	}

	fn bind_vertex_array(&self) {
		let gl = self.gl.as_ref().unwrap();
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
