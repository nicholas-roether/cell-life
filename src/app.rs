use std::{
	rc::Rc,
	thread,
	time::{Duration, SystemTime}
};

use winit::event_loop::EventLoopProxy;

use crate::{
	render::{layers::dots::DotsLayer, Renderer},
	simulation::Simulation,
	window::Window
};

trait SyncHandle: Send {
	fn advance(&mut self, dt: f64);
}

struct WindowThreadSyncHandle {
	proxy: EventLoopProxy<f64>
}

impl WindowThreadSyncHandle {
	fn new(proxy: EventLoopProxy<f64>) -> Self {
		Self { proxy }
	}
}

impl SyncHandle for WindowThreadSyncHandle {
	fn advance(&mut self, dt: f64) {
		self.proxy
			.send_event(dt)
			.unwrap_or_else(|err| eprintln!("Failed to send window update event: {err}"))
	}
}

struct WindowThread {
	window: Window
}

const APP_NAME: &str = "Cell Life";

impl WindowThread {
	fn new() -> Self {
		let window = Window::new(APP_NAME, |gl| {
			let mut renderer = Renderer::new(Rc::clone(&gl));
			renderer.push_layer(DotsLayer::new(gl, Simulation::new()));
			renderer
		});
		Self { window }
	}

	fn sync_handle(&self) -> WindowThreadSyncHandle {
		WindowThreadSyncHandle::new(self.window.proxy())
	}

	fn start_sync(self) {
		self.window.run()
	}
}

struct TimingThread {
	handles: Vec<Box<dyn SyncHandle>>
}

const TPS: f64 = 60.0;

impl TimingThread {
	fn new() -> Self {
		Self {
			handles: Vec::new()
		}
	}

	fn add_handle<H: SyncHandle + 'static>(&mut self, handle: H) {
		self.handles.push(Box::new(handle));
	}

	fn start(mut self) {
		thread::spawn(move || loop {
			let now = SystemTime::now();
			thread::sleep(Duration::from_secs_f64(1.0 / TPS));
			let Ok(dt) = now.elapsed() else { continue };
			self.tick(dt.as_secs_f64());
		});
	}

	fn tick(&mut self, dt: f64) {
		for handle in &mut self.handles {
			handle.advance(dt)
		}
	}
}

pub struct App {
	timing_thread: TimingThread,
	window_thread: WindowThread
}

impl App {
	pub fn new() -> Self {
		let mut timing_thread = TimingThread::new();
		let window_thread = WindowThread::new();

		timing_thread.add_handle(window_thread.sync_handle());

		Self {
			window_thread,
			timing_thread
		}
	}

	pub fn start(self) {
		self.timing_thread.start();
		self.window_thread.start_sync();
	}
}
