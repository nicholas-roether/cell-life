use std::io::Write;

use glow::HasContext;

use super::context::GraphicsContext;

#[derive(Debug)]
pub struct Buffer {
	ctx: GraphicsContext,
	length: usize,
	capacity: usize,
	target: u32,
	native_buffer: glow::NativeBuffer
}

macro_rules! to_binary {
	($slice:expr, $type:ty) => {
		::std::slice::from_raw_parts(
			($slice as *const _) as *const u8,
			$slice.len() * ::std::mem::size_of::<$type>()
		)
	};
}

const INITIAL_SIZE: usize = 64;
const RESIZE_FACTOR: usize = 2;

impl Buffer {
	fn new(ctx: GraphicsContext, target: u32) -> Self {
		let native_buffer = unsafe { ctx.gl.create_buffer() }.expect("Failed to create buffer");
		Self {
			ctx,
			length: 0,
			capacity: 0,
			target,
			native_buffer
		}
	}

	pub fn len(&self) -> usize {
		self.length
	}

	pub fn bind_base(&self, index: u32) {
		unsafe {
			self.ctx
				.gl
				.bind_buffer_base(self.target, index, Some(self.native_buffer))
		}
	}

	pub fn set_data<T>(&mut self, data: &[T], usage: u32) {
		let bin_data = unsafe { to_binary!(data, T) };
		self.bind();
		unsafe {
			self.ctx
				.gl
				.buffer_data_u8_slice(self.target, bin_data, usage)
		}
		self.length = data.len();
		self.capacity = data.len();
	}

	pub fn make_writer(&mut self, usage: u32) -> BufferWriter<'_> {
		BufferWriter::new(self, usage)
	}

	pub fn write(&mut self, data: &[u8], offset: usize, usage: u32) -> usize {
		let new_length = self.len() + data.len();
		if new_length > self.capacity {
			self.grow(usage);
		} else if new_length > INITIAL_SIZE && new_length < self.capacity / RESIZE_FACTOR {
			self.shrink(usage);
		}
		self.bind();
		unsafe {
			self.ctx
				.gl
				.buffer_sub_data_u8_slice(self.target, offset as i32, data);
		};
		self.length = new_length;
		self.len()
	}

	pub fn bind(&self) {
		unsafe {
			self.ctx
				.gl
				.bind_buffer(self.target, Some(self.native_buffer))
		}
	}

	fn resize(&mut self, new_capacity: usize, usage: u32) {
		unsafe {
			self.ctx
				.gl
				.bind_buffer(glow::COPY_READ_BUFFER, Some(self.native_buffer));
			let new_buffer = self
				.ctx
				.gl
				.create_buffer()
				.expect("Failed to create copy buffer");
			self.ctx
				.gl
				.bind_buffer(glow::COPY_WRITE_BUFFER, Some(new_buffer));
			self.ctx
				.gl
				.buffer_data_size(glow::COPY_WRITE_BUFFER, new_capacity as i32, usage);
			self.ctx.gl.copy_buffer_sub_data(
				glow::COPY_READ_BUFFER,
				glow::COPY_WRITE_BUFFER,
				0,
				0,
				self.length as i32
			);
			self.ctx.gl.delete_buffer(self.native_buffer);
			self.ctx.gl.bind_buffer(glow::COPY_WRITE_BUFFER, None);
			self.native_buffer = new_buffer;
			self.capacity = new_capacity;
		}
	}

	fn grow(&mut self, usage: u32) {
		let new_cap = if self.capacity == 0 {
			INITIAL_SIZE
		} else {
			self.capacity * RESIZE_FACTOR
		};
		self.resize(new_cap, usage);
	}

	fn shrink(&mut self, usage: u32) {
		let new_cap = if self.capacity == 0 {
			0
		} else {
			self.capacity / RESIZE_FACTOR
		};
		self.resize(new_cap, usage);
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		unsafe { self.ctx.gl.delete_buffer(self.native_buffer) }
	}
}

pub struct BufferWriter<'a> {
	buffer: &'a mut Buffer,
	length: usize,
	usage: u32
}

impl<'a> BufferWriter<'a> {
	fn new(buffer: &'a mut Buffer, usage: u32) -> Self {
		Self {
			buffer,
			length: 0,
			usage
		}
	}
}

impl<'a> Write for BufferWriter<'a> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.buffer.write(buf, self.length, self.usage);
		self.length += buf.len();
		Ok(buf.len())
	}
	fn flush(&mut self) -> std::io::Result<()> {
		Ok(())
	}
}

impl GraphicsContext {
	pub fn make_buffer(&self, target: u32) -> Buffer {
		Buffer::new(self.clone(), target)
	}
}
