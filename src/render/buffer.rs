use std::{io::Write, rc::Rc};

use glow::HasContext;

#[derive(Debug)]
pub struct Buffer {
	gl: Rc<glow::Context>,
	target: u32,
	native_buffer: glow::NativeBuffer
}

macro_rules! to_binary {
	($slice:expr, $type:ty) => {
		::std::slice::from_raw_parts(
			(&$slice as *const _) as *const u8,
			$slice.len() * ::std::mem::size_of::<$type>()
		)
	};
}

impl Buffer {
	pub fn new(gl: Rc<glow::Context>, target: u32) -> Self {
		let native_buffer = unsafe { gl.create_buffer() }.expect("Failed to create buffer");
		Self {
			gl,
			target,
			native_buffer
		}
	}

	pub fn bind(&self) {
		unsafe { self.gl.bind_buffer(self.target, Some(self.native_buffer)) }
	}

	pub fn write<T>(&self, data: &[T], usage: u32) {
		let bin_data = unsafe { to_binary!(data, T) };
		self.bind();
		unsafe { self.gl.buffer_data_u8_slice(self.target, bin_data, usage) }
	}

	pub fn make_writer(&self, usage: u32) -> BufferWriter<'_> {
		BufferWriter::new(self, usage)
	}

	pub fn unbind(&self) {
		unsafe { self.gl.bind_buffer(self.target, None) }
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		unsafe { self.gl.delete_buffer(self.native_buffer) }
	}
}

pub struct BufferWriter<'a> {
	buffer: &'a Buffer,
	data: Vec<u8>,
	usage: u32
}

impl<'a> BufferWriter<'a> {
	fn new(buffer: &'a Buffer, usage: u32) -> Self {
		Self {
			buffer,
			data: Vec::new(),
			usage
		}
	}
}

impl<'a> Write for BufferWriter<'a> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.data.extend_from_slice(buf);
		Ok(self.data.len())
	}

	fn flush(&mut self) -> std::io::Result<()> {
		self.buffer.write(&self.data, self.usage);
		Ok(())
	}
}
