use std::{mem::size_of, rc::Rc};

use glow::{HasContext, VertexArray};

#[derive(Debug)]
struct VertexAttribute {
	size: u32,
	data_type: u32,
	type_size: usize
}

impl VertexAttribute {
	fn new(size: u32, data_type: u32) -> Self {
		let type_size = match data_type {
			glow::FLOAT => size_of::<f32>(),
			glow::DOUBLE => size_of::<f64>(),
			glow::INT => size_of::<i32>(),
			_ => panic!("Unsupported data type")
		};
		Self {
			size,
			data_type,
			type_size
		}
	}

	fn register(&self, gl: &glow::Context, index: usize, vertex_size: usize, offset: usize) {
		unsafe {
			match self.data_type {
				glow::FLOAT => gl.vertex_attrib_pointer_f32(
					index as u32,
					self.size as i32,
					self.data_type,
					false,
					vertex_size as i32,
					offset as i32
				),
				glow::DOUBLE => gl.vertex_attrib_pointer_f64(
					index as u32,
					self.size as i32,
					self.data_type,
					vertex_size as i32,
					offset as i32
				),
				glow::INT => gl.vertex_attrib_pointer_i32(
					index as u32,
					self.size as i32,
					self.data_type,
					vertex_size as i32,
					offset as i32
				),
				_ => unreachable!()
			}
		}
	}
}

#[derive(Debug)]
pub struct VertexModel {
	gl: Rc<glow::Context>,
	vertex_array: VertexArray,
	vertex_size: usize,
	attributes: Vec<VertexAttribute>
}

impl VertexModel {
	pub fn new(gl: Rc<glow::Context>) -> Self {
		let vertex_array = unsafe {
			gl.create_vertex_array()
				.expect("Failed to create vertex array")
		};
		Self {
			gl: Rc::clone(&gl),
			vertex_array,
			vertex_size: 0,
			attributes: Vec::new()
		}
	}

	pub fn bind(&self) {
		unsafe {
			self.gl.bind_vertex_array(Some(self.vertex_array));
		}
	}

	pub fn add_attribute(&mut self, size: u32, data_type: u32) {
		let attr = VertexAttribute::new(size, data_type);
		self.vertex_size += attr.type_size * attr.size as usize;
		self.attributes.push(attr);
	}

	pub fn apply(&self) {
		self.bind();
		let mut offset = 0;
		for (i, attr) in self.attributes.iter().enumerate() {
			attr.register(&self.gl, i, self.vertex_size, offset);
			unsafe { self.gl.enable_vertex_attrib_array(i as u32) }
			offset += attr.size as usize * attr.type_size;
		}
	}
}

impl Drop for VertexModel {
	fn drop(&mut self) {
		unsafe { self.gl.delete_vertex_array(self.vertex_array) }
	}
}
