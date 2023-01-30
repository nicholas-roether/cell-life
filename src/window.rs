use std::{ffi::CString, num::NonZeroU32};

use glow::HasContext;
use glutin::{
	config::{Config, ConfigTemplateBuilder},
	context::{ContextAttributesBuilder, PossiblyCurrentContext},
	display::{Display, GetGlDisplay},
	prelude::{GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor},
	surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface}
};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use winit::{
	dpi::LogicalSize,
	event::{Event, WindowEvent},
	event_loop::EventLoop,
	window::WindowBuilder
};

pub trait Renderer {
	fn init(&mut self, gl: &glow::Context);

	fn draw(&mut self, gl: &glow::Context);
}

const MIN_WIDTH: u32 = 360;
const MIN_HEIGHT: u32 = 240;

pub struct Window {
	window: winit::window::Window,
	event_loop: Option<EventLoop<()>>,
	gl: glow::Context,
	gl_surface: Surface<WindowSurface>,
	gl_context: PossiblyCurrentContext,
	renderer: Box<dyn Renderer>
}

impl Window {
	pub fn new(title: &str, mut renderer: Box<dyn Renderer>) -> Self {
		let event_loop = EventLoop::new();
		let (window, gl_config) = Self::create_window(title, &event_loop);
		let gl_display = gl_config.display();
		let gl_surface = Self::create_surface(&window, &gl_display, &gl_config);
		let gl_context = Self::create_active_context(&window, &gl_display, &gl_config, &gl_surface);
		let gl = Self::gl(&gl_display);

		renderer.init(&gl);

		Self {
			window,
			renderer,
			event_loop: Some(event_loop),
			gl,
			gl_surface,
			gl_context
		}
	}

	pub fn run(mut self) {
		self.event_loop
			.take()
			.expect("Window is already running")
			.run(move |window_event, _window_target, control_flow| {
				control_flow.set_wait();
				match window_event {
					Event::WindowEvent { event, .. } => match event {
						WindowEvent::Resized(size) => self.resize(size.width, size.height),
						WindowEvent::CloseRequested => {
							control_flow.set_exit();
						}
						_ => ()
					},
					Event::RedrawRequested(_) => {
						self.renderer.draw(&self.gl);
					}
					Event::RedrawEventsCleared => {
						self.window.request_redraw();
						self.gl_surface.swap_buffers(&self.gl_context).unwrap();
					}
					_ => ()
				}
			})
	}

	fn create_window(title: &str, event_loop: &EventLoop<()>) -> (winit::window::Window, Config) {
		let window_builder = WindowBuilder::new()
			.with_title(title)
			.with_min_inner_size(LogicalSize::new(MIN_WIDTH, MIN_HEIGHT));
		let template = ConfigTemplateBuilder::default();
		let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
		let (mut window_opt, gl_config) = display_builder
			.build(event_loop, template, Self::select_gl_config)
			.expect("Failed to create window");

		let window = window_opt.take().expect("Window was None");
		(window, gl_config)
	}

	fn create_surface(
		window: &winit::window::Window,
		gl_display: &Display,
		gl_config: &Config
	) -> Surface<WindowSurface> {
		let surface_attributes =
			window.build_surface_attributes(SurfaceAttributesBuilder::default());
		unsafe { gl_display.create_window_surface(gl_config, &surface_attributes) }
			.expect("Failed to create window surface")
	}

	fn create_active_context(
		window: &winit::window::Window,
		gl_display: &Display,
		gl_config: &Config,
		gl_surface: &Surface<WindowSurface>
	) -> PossiblyCurrentContext {
		let raw_window_handle = window.raw_window_handle();
		let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));

		let gl_inactive_context =
			unsafe { gl_display.create_context(gl_config, &context_attributes) }
				.expect("Failed to create context");

		gl_inactive_context
			.make_current(gl_surface)
			.expect("Failed to activate OpenGL context")
	}

	fn gl(gl_display: &Display) -> glow::Context {
		unsafe {
			glow::Context::from_loader_function(|symbol| {
				gl_display.get_proc_address(&CString::new(symbol).unwrap())
			})
		}
	}

	fn select_gl_config<'a>(configs: Box<dyn Iterator<Item = Config> + 'a>) -> Config {
		configs
			.reduce(|acc, config| {
				if config.num_samples() > acc.num_samples() {
					config
				} else {
					acc
				}
			})
			.expect("No suitable OpenGL config found")
	}

	fn resize(&self, width: u32, height: u32) {
		if width == 0 || height == 0 {
			return;
		}
		self.gl_surface.resize(
			&self.gl_context,
			NonZeroU32::new(width).unwrap(),
			NonZeroU32::new(height).unwrap()
		);
		unsafe { self.gl.viewport(0, 0, width as i32, height as i32) };
	}
}
