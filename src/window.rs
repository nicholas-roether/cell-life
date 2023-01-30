use std::num::NonZeroU32;

use glutin::{
	config::{Config, ConfigTemplateBuilder},
	context::{ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext},
	display::GetGlDisplay,
	prelude::{GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor},
	surface::{GlSurface, Surface, WindowSurface}
};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use winit::{
	event::{Event, WindowEvent},
	event_loop::{EventLoop, EventLoopBuilder},
	window::WindowBuilder
};

struct WindowState {
	gl_context: PossiblyCurrentContext,
	gl_surface: Surface<WindowSurface>
}

pub struct Window {
	window: Option<winit::window::Window>,
	event_loop: EventLoop<()>,
	gl_config: Config,
	gl_inactive_context: NotCurrentContext
}

impl Window {
	pub fn new(title: &str) -> Self {
		let event_loop = EventLoopBuilder::new().build();
		let window_builder = WindowBuilder::new().with_title(title);
		let template = ConfigTemplateBuilder::new();
		let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
		let (mut window, gl_config) = display_builder
			.build(&event_loop, template, Self::select_gl_config)
			.expect("Failed to create window");

		let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());
		let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);
		let gl_display = gl_config.display();
		let gl_context = unsafe { gl_display.create_context(&gl_config, &context_attributes) }
			.expect("Failed to create context");

		Self {
			window,
			event_loop,
			gl_config,
			gl_inactive_context: gl_context
		}
	}

	pub fn run(self) {
		let mut state: Option<WindowState> = None;
		self.event_loop
			.run(move |event, window_target, control_flow| {
				control_flow.set_wait();
				match event {
					Event::Resumed => {
						// state = Some(self.activate());
					}
					Event::WindowEvent { event, .. } => match event {
						WindowEvent::Resized(size) => {
							if size.width == 0 || size.height == 0 {
								return;
							}
							let Some(WindowState {
								gl_context,
								gl_surface
							}) = &state else {
								return;
							};
							gl_surface.resize(
								gl_context,
								NonZeroU32::new(size.width).unwrap(),
								NonZeroU32::new(size.height).unwrap()
							);
						}
						WindowEvent::CloseRequested => {
							control_flow.set_exit();
						}
						_ => ()
					},
					_ => ()
				}
			})
	}

	fn select_gl_config(configs: Box<dyn Iterator<Item = Config>>) -> Config {
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

	fn activate(
		window: Option<winit::window::Window>,
		gl_inactive_context: NotCurrentContext,
		gl_config: &Config
	) -> WindowState {
		let attrs = window.unwrap().build_surface_attributes(Default::default());

		let gl_surface = unsafe {
			gl_config
				.display()
				.create_window_surface(&gl_config, &attrs)
		}
		.expect("Failed to create surface");
		let gl_context = gl_inactive_context
			.make_current(&gl_surface)
			.expect("Failed to active OpenGL context");
		WindowState {
			gl_surface,
			gl_context
		}
	}
}
