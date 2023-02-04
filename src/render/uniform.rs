use glam::{IVec2, IVec3, IVec4, Mat2, Mat3, Mat4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec4};
use glow::HasContext;

use super::{shader::ShaderProgram, GraphicsContext};

struct Uniform {
	ctx: GraphicsContext,
	location: Option<glow::UniformLocation>
}

impl Uniform {
	fn new(ctx: GraphicsContext, location: Option<glow::UniformLocation>) -> Self {
		Self { ctx, location }
	}
}

trait ToUniformValue<'a, T> {
	fn to_uniform_value(&'a self) -> T;
}

macro_rules! define_uniform_conversion {
	($ty:ty, $out_ty:ty, $conv_fn:ident) => {
		impl<'a> ToUniformValue<'a, $out_ty> for $ty {
			fn to_uniform_value(&'a self) -> $out_ty {
				self.$conv_fn()
			}
		}
	};
}

define_uniform_conversion!(f32, f32, clone);
define_uniform_conversion!(Vec2, &'a [f32], as_ref);
define_uniform_conversion!(Vec3, &'a [f32], as_ref);
define_uniform_conversion!(Vec4, &'a [f32], as_ref);
define_uniform_conversion!(i32, i32, clone);
define_uniform_conversion!(IVec2, &'a [i32], as_ref);
define_uniform_conversion!(IVec3, &'a [i32], as_ref);
define_uniform_conversion!(IVec4, &'a [i32], as_ref);
define_uniform_conversion!(u32, u32, clone);
define_uniform_conversion!(UVec2, &'a [u32], as_ref);
define_uniform_conversion!(UVec3, &'a [u32], as_ref);
define_uniform_conversion!(UVec4, &'a [u32], as_ref);
define_uniform_conversion!(Mat2, &'a [f32], as_ref);
define_uniform_conversion!(Mat3, &'a [f32], as_ref);
define_uniform_conversion!(Mat4, &'a [f32], as_ref);

macro_rules! get_uniform_location {
	($name:expr, $loc_opt:expr) => {{
		let Some(location) = $loc_opt else {
								eprintln!("Failed to get uniform location for '{}'", $name);
								return;
							};
		location
	}};
}

macro_rules! define_uniform_variant {
	($name:ident, $val_type:ty, $fn_name:ident) => {
		pub struct $name(Uniform);

		#[allow(unused)]
		impl $name {
			pub fn set(&self, value: $val_type) {
				let location = get_uniform_location!(value, &self.0.location);
				unsafe {
					self.0
						.ctx
						.gl
						.$fn_name(Some(location), value.to_uniform_value())
				}
			}
		}
	};
	($name:ident, $val_type:ty, $fn_name:ident, $arg2:expr) => {
		pub struct $name(Uniform);

		#[allow(unused)]
		impl $name {
			pub fn set(&self, value: $val_type) {
				unsafe {
					let location = get_uniform_location!(value, &self.0.location);
					self.0
						.ctx
						.gl
						.$fn_name(Some(location), $arg2, value.to_uniform_value())
				}
			}
		}
	};
}

define_uniform_variant!(UniformF32, f32, uniform_1_f32);
define_uniform_variant!(UniformVec2, Vec2, uniform_2_f32_slice);
define_uniform_variant!(UniformVec3, Vec3, uniform_3_f32_slice);
define_uniform_variant!(UniformVec4, Vec4, uniform_4_f32_slice);
define_uniform_variant!(UniformI32, i32, uniform_1_i32);
define_uniform_variant!(UniformIVec2, IVec2, uniform_2_i32_slice);
define_uniform_variant!(UniformIVec3, IVec3, uniform_3_i32_slice);
define_uniform_variant!(UniformIVec4, IVec4, uniform_4_i32_slice);
define_uniform_variant!(UniformU32, u32, uniform_1_u32);
define_uniform_variant!(UniformUVec2, UVec2, uniform_2_u32_slice);
define_uniform_variant!(UniformUVec3, UVec3, uniform_3_u32_slice);
define_uniform_variant!(UniformUVec4, UVec4, uniform_4_u32_slice);
define_uniform_variant!(UniformMat2, Mat2, uniform_matrix_2_f32_slice, false);
define_uniform_variant!(UniformMat3, Mat3, uniform_matrix_3_f32_slice, false);
define_uniform_variant!(UniformMat4, Mat4, uniform_matrix_3_f32_slice, false);

macro_rules! uniform_getter {
	($fn_name:ident, $uniform_type:ty, $uniform_constr:expr) => {
		pub fn $fn_name(&self, name: &str) -> $uniform_type {
			$uniform_constr(Uniform::new(
				self.ctx.clone(),
				self.get_uniform_location(name)
			))
		}
	};
}

#[allow(unused)]
impl ShaderProgram {
	uniform_getter!(get_uniform_f32, UniformF32, UniformF32);
	uniform_getter!(get_uniform_vec2, UniformVec2, UniformVec2);
	uniform_getter!(get_uniform_vec3, UniformVec3, UniformVec3);
	uniform_getter!(get_uniform_vec4, UniformVec4, UniformVec4);
	uniform_getter!(get_uniform_i32, UniformI32, UniformI32);
	uniform_getter!(get_uniform_ivec2, UniformIVec2, UniformIVec2);
	uniform_getter!(get_uniform_ivec3, UniformIVec3, UniformIVec3);
	uniform_getter!(get_uniform_ivec4, UniformIVec4, UniformIVec4);
	uniform_getter!(get_uniform_u32, UniformU32, UniformU32);
	uniform_getter!(get_uniform_uvec2, UniformUVec2, UniformUVec2);
	uniform_getter!(get_uniform_uvec3, UniformUVec3, UniformUVec3);
	uniform_getter!(get_uniform_uvec4, UniformUVec4, UniformUVec4);
	uniform_getter!(get_uniform_mat2, UniformMat2, UniformMat2);
	uniform_getter!(get_uniform_mat3, UniformMat3, UniformMat3);
	uniform_getter!(get_uniform_mat4, UniformMat4, UniformMat4);
}
