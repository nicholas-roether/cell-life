use std::{ffi::CString, mem, num::NonZeroU32, rc::Rc};

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
	dpi::{LogicalSize, PhysicalSize},
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy},
	platform::unix::WindowBuilderExtUnix,
	window::{Theme, WindowBuilder}
};

pub trait Renderer {
	fn draw(&mut self, size: LogicalSize<f32>);
}

const MIN_WIDTH: u32 = 360;
const MIN_HEIGHT: u32 = 240;

pub struct Window {
	window: winit::window::Window,
	event_loop: Option<EventLoop<f64>>,
	size: LogicalSize<f32>,
	gl: Rc<glow::Context>,
	gl_surface: Surface<WindowSurface>,
	gl_context: PossiblyCurrentContext,
	renderer: Option<Box<dyn Renderer>>
}

impl Window {
	pub fn new<R: Renderer + 'static, RB: FnOnce(Rc<glow::Context>) -> R>(
		title: &str,
		renderer_builder: RB
	) -> Self {
		let event_loop = EventLoopBuilder::<f64>::with_user_event().build();
		let (window, gl_config) = Self::create_window(title, &event_loop);
		let gl_display = gl_config.display();
		let gl_surface = Self::create_surface(&window, &gl_display, &gl_config);
		let gl_context = Self::create_active_context(&window, &gl_display, &gl_config, &gl_surface);
		let gl = Self::gl(&gl_display);
		let size = Self::logical_size(&window, window.inner_size());
		let renderer = Box::new(renderer_builder(Rc::clone(&gl)));

		Self {
			window,
			size,
			renderer: Some(renderer),
			event_loop: Some(event_loop),
			gl,
			gl_surface,
			gl_context
		}
	}

	pub fn proxy(&self) -> EventLoopProxy<f64> {
		self.event_loop
			.as_ref()
			.expect("Event loop was already destroyed!")
			.create_proxy()
	}

	pub fn run(mut self) {
		self.event_loop
			.take()
			.expect("Window is already running")
			.run(move |event, _window_target, control_flow| self.handle_event(event, control_flow));
	}

	fn handle_event(&mut self, event: Event<f64>, control_flow: &mut ControlFlow) {
		control_flow.set_wait();
		match event {
			Event::WindowEvent { event, .. } => self.handle_window_event(event, control_flow),
			Event::RedrawRequested(_) => {
				if let Some(renderer) = &mut self.renderer {
					renderer.draw(self.size);
					self.gl_surface.swap_buffers(&self.gl_context).unwrap();
				}
			}
			Event::UserEvent(_) => self.window.request_redraw(),
			_ => ()
		}
	}

	fn handle_window_event(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
		match event {
			WindowEvent::Resized(size) => self.resize(size),
			WindowEvent::CloseRequested => {
				control_flow.set_exit();
				if let Some(renderer) = self.renderer.take() {
					mem::drop(renderer); // Ensure the renderer is dropped before the OpenGL context is destroyed
				}
			}
			_ => ()
		}
	}

	fn create_window(title: &str, event_loop: &EventLoop<f64>) -> (winit::window::Window, Config) {
		let window_builder = WindowBuilder::new()
			.with_title(title)
			.with_gtk_theme_variant(String::from("dark"))
			.with_wayland_csd_theme(Theme::Dark)
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

	fn gl(gl_display: &Display) -> Rc<glow::Context> {
		Rc::new(unsafe {
			glow::Context::from_loader_function(|symbol| {
				gl_display.get_proc_address(&CString::new(symbol).unwrap())
			})
		})
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

	fn logical_size(
		window: &winit::window::Window,
		physical_size: PhysicalSize<u32>
	) -> LogicalSize<f32> {
		LogicalSize {
			width: (physical_size.width as f64 / window.scale_factor()) as f32,
			height: (physical_size.height as f64 / window.scale_factor()) as f32
		}
	}

	fn resize(&mut self, physical_size: PhysicalSize<u32>) {
		if physical_size.width == 0 || physical_size.height == 0 {
			return;
		}
		self.size = Self::logical_size(&self.window, physical_size);
		self.gl_surface.resize(
			&self.gl_context,
			NonZeroU32::new(self.size.width as u32).unwrap(),
			NonZeroU32::new(self.size.height as u32).unwrap()
		);
		unsafe {
			self.gl
				.viewport(0, 0, self.size.width as i32, self.size.height as i32)
		};
	}
}
