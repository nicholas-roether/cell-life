use std::{
	rc::Rc,
	sync::{
		mpsc::{self, Receiver, Sender},
		Arc, RwLock
	},
	thread,
	time::{Duration, SystemTime}
};

use winit::event_loop::EventLoopProxy;

use crate::{
	render::{layers::dots::DotsLayer, Renderer},
	simulation::{Simulation, Tick},
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
	fn new(simulation: Arc<RwLock<Simulation>>) -> Self {
		let window = Window::new(APP_NAME, |gl| {
			let mut renderer = Renderer::new(Rc::clone(&gl));
			renderer.push_layer(DotsLayer::new(gl, simulation));
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

struct ChannelSyncHandle {
	sender: Sender<f64>
}

impl ChannelSyncHandle {
	fn new(sender: Sender<f64>) -> Self {
		Self { sender }
	}
}

impl SyncHandle for ChannelSyncHandle {
	fn advance(&mut self, dt: f64) {
		self.sender
			.send(dt)
			.unwrap_or_else(|err| eprintln!("Failed to send sync event to thread: {err}"))
	}
}

struct SyncedThread {
	receiver: Option<Receiver<f64>>,
	func: Box<dyn FnOnce(Receiver<f64>) + Send + 'static>,
	sender: Sender<f64>
}

impl SyncedThread {
	fn new<F: FnOnce(Receiver<f64>) + Send + 'static>(func: F) -> Self {
		let (sender, receiver) = mpsc::channel::<f64>();
		Self {
			sender,
			func: Box::new(func),
			receiver: Some(receiver)
		}
	}

	fn start(mut self) {
		thread::spawn(move || (self.func)(self.receiver.take().unwrap()));
	}

	fn sync_handle(&self) -> ChannelSyncHandle {
		ChannelSyncHandle::new(self.sender.clone())
	}
}

struct SimThread {
	synced_thread: SyncedThread
}

impl SimThread {
	fn new(simulation: Arc<RwLock<Simulation>>) -> Self {
		let synced_thread = SyncedThread::new(move |recv| {
			for dt in recv {
				let  Ok(mut sim) = simulation.write() else {
					eprintln!("Failed to acquire lock on simulation");
					continue;
				};
				sim.tick(dt)
			}
		});
		Self { synced_thread }
	}

	fn start(self) {
		self.synced_thread.start();
	}

	fn sync_handle(&self) -> ChannelSyncHandle {
		self.synced_thread.sync_handle()
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
	window_thread: WindowThread,
	sim_thread: SimThread
}

impl App {
	pub fn new() -> Self {
		let simulation = Arc::new(RwLock::new(Simulation::new()));

		let mut timing_thread = TimingThread::new();
		let window_thread = WindowThread::new(Arc::clone(&simulation));
		let sim_thread = SimThread::new(simulation);

		timing_thread.add_handle(sim_thread.sync_handle());
		timing_thread.add_handle(window_thread.sync_handle());

		Self {
			sim_thread,
			window_thread,
			timing_thread
		}
	}

	pub fn start(self) {
		self.timing_thread.start();
		self.sim_thread.start();
		self.window_thread.start_sync();
	}
}
