use std::{mem::size_of, slice};

use glow::HasContext;

use crate::window;

pub struct Renderer;

type Vertex = [f32; 3];

const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

const VERTEX_SHADER: &str = include_str!("./shaders/test.vert.glsl");
const FRAGMENT_SHADER: &str = include_str!("./shaders/test.frag.glsl");

impl window::Renderer for Renderer {
	fn init(&mut self, gl: &glow::Context) {
		unsafe {
			let vertex_array = gl
				.create_vertex_array()
				.expect("Failed to create vertex array");
			gl.bind_vertex_array(Some(vertex_array));

			let vertex_buffer = gl.create_buffer().expect("Failed to create vertex buffer");
			gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
			gl.buffer_data_u8_slice(
				glow::ARRAY_BUFFER,
				slice::from_raw_parts(
					(&VERTICES as *const Vertex) as *const u8,
					VERTICES.len() * size_of::<Vertex>()
				),
				glow::STATIC_DRAW
			);

			gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, size_of::<Vertex>() as i32, 0);

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

	fn draw(&mut self, gl: &glow::Context) {
		unsafe {
			gl.clear(glow::COLOR_BUFFER_BIT);
			gl.draw_arrays(glow::TRIANGLES, 0, 3);
		}
	}
}

impl Renderer {
	pub fn new() -> Self {
		Self {}
	}
}
