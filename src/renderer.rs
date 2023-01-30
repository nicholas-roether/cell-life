use std::{mem::size_of, slice};

use glow::HasContext;
use winit::dpi::LogicalSize;

use crate::window;

pub struct Renderer;

#[allow(unused)]
#[derive(Debug)]
struct Vertex {
	u: f32,
	v: f32,
	x: f32,
	y: f32
}

const VERTEX_SHADER: &str = include_str!("./shaders/test.vert.glsl");
const FRAGMENT_SHADER: &str = include_str!("./shaders/test.frag.glsl");

const NUM_VERTICES: usize = 4;

impl window::Renderer for Renderer {
	fn init(&mut self, gl: &glow::Context) {
		unsafe {
			let vertex_array = gl
				.create_vertex_array()
				.expect("Failed to create vertex array");
			gl.bind_vertex_array(Some(vertex_array));

			let vertex_buffer = gl.create_buffer().expect("Failed to create vertex buffer");
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));

			gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, size_of::<Vertex>() as i32, 0);
			gl.vertex_attrib_pointer_f32(
				1,
				2,
				glow::FLOAT,
				false,
				size_of::<Vertex>() as i32,
				2 * size_of::<f32>() as i32
			);
			gl.enable_vertex_attrib_array(0);
			gl.enable_vertex_attrib_array(1);

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

	fn draw(&mut self, gl: &glow::Context, LogicalSize { width, height }: LogicalSize<f32>) {
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
	pub fn new() -> Self {
		Self {}
	}

	fn vertex_data(width: f32, height: f32) -> [u8; size_of::<Vertex>() * NUM_VERTICES] {
		let vertices = Self::generate_vertices(width, height);
		let slice = unsafe {
			slice::from_raw_parts(
				(&vertices as *const Vertex) as *const u8,
				vertices.len() * size_of::<Vertex>()
			)
		};
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
