use glow::{HasContext, NativeProgram, NativeShader};

use super::context::GraphicsContext;

pub struct Shader {
	ctx: GraphicsContext,
	native_shader: NativeShader
}

impl Shader {
	fn new(ctx: GraphicsContext, shader_type: u32, source: &str) -> Self {
		unsafe {
			let native_shader = ctx
				.gl
				.create_shader(shader_type)
				.expect("Failed to create shader");
			ctx.gl.shader_source(native_shader, source);
			ctx.gl.compile_shader(native_shader);

			if !ctx.gl.get_shader_compile_status(native_shader) {
				panic!(
					"Failed to compile shader: {}",
					ctx.gl.get_shader_info_log(native_shader)
				);
			}

			Self { ctx, native_shader }
		}
	}

	fn attach_to(&self, program: NativeProgram) {
		unsafe { self.ctx.gl.attach_shader(program, self.native_shader) }
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe {
			self.ctx.gl.delete_shader(self.native_shader);
		}
	}
}

pub struct ShaderProgram {
	ctx: GraphicsContext,
	native_program: NativeProgram
}

impl ShaderProgram {
	fn new(ctx: GraphicsContext, shaders: Vec<Shader>) -> Self {
		unsafe {
			let native_program = ctx
				.gl
				.create_program()
				.expect("Failed to create shader program");
			for shader in shaders {
				shader.attach_to(native_program);
			}
			ctx.gl.link_program(native_program);
			if !ctx.gl.get_program_link_status(native_program) {
				panic!(
					"Failed to link shader program: {}",
					ctx.gl.get_program_info_log(native_program)
				);
			}
			Self {
				ctx,
				native_program
			}
		}
	}

	pub fn activate(&self) {
		unsafe { self.ctx.gl.use_program(Some(self.native_program)) }
	}
}

impl Drop for ShaderProgram {
	fn drop(&mut self) {
		unsafe { self.ctx.gl.delete_program(self.native_program) }
	}
}

impl GraphicsContext {
	pub fn make_shader(&self, shader_type: u32, source: &str) -> Shader {
		Shader::new(self.clone(), shader_type, source)
	}

	pub fn make_program(&self, shaders: Vec<Shader>) -> ShaderProgram {
		ShaderProgram::new(self.clone(), shaders)
	}
}
