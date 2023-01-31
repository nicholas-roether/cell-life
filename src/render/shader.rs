use std::rc::Rc;

use glow::{HasContext, NativeProgram, NativeShader};

pub struct Shader {
	gl: Rc<glow::Context>,
	native_shader: NativeShader
}

impl Shader {
	pub fn new(gl: Rc<glow::Context>, shader_type: u32, source: &str) -> Self {
		unsafe {
			let native_shader = gl
				.create_shader(shader_type)
				.expect("Failed to create shader");
			gl.shader_source(native_shader, source);
			gl.compile_shader(native_shader);

			if !gl.get_shader_compile_status(native_shader) {
				panic!(
					"Failed to compile shader: {}",
					gl.get_shader_info_log(native_shader)
				);
			}

			Self { gl, native_shader }
		}
	}

	fn attach_to(&self, program: NativeProgram) {
		unsafe { self.gl.attach_shader(program, self.native_shader) }
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_shader(self.native_shader);
		}
	}
}

pub struct ShaderProgram {
	gl: Rc<glow::Context>,
	native_program: NativeProgram
}

impl ShaderProgram {
	pub fn new(gl: Rc<glow::Context>, shaders: Vec<Shader>) -> Self {
		unsafe {
			let native_program = gl
				.create_program()
				.expect("Failed to create shader program");
			for shader in shaders {
				shader.attach_to(native_program);
			}
			gl.link_program(native_program);
			if !gl.get_program_link_status(native_program) {
				panic!(
					"Failed to link shader program: {}",
					gl.get_program_info_log(native_program)
				);
			}
			Self { gl, native_program }
		}
	}

	pub fn activate(&self) {
		unsafe { self.gl.use_program(Some(self.native_program)) }
	}
}

impl Drop for ShaderProgram {
	fn drop(&mut self) {
		unsafe { self.gl.delete_program(self.native_program) }
	}
}
