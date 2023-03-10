use std::{
	sync::{
		mpsc::{self, Receiver, Sender},
		Arc, Mutex
	},
	thread,
	time::{Duration, SystemTime}
};

use glam::{vec2, vec3};
use winit::event_loop::EventLoopProxy;

use crate::{
	particles::ParticleSystem,
	render::{
		layers::{dots::DotsLayer, particles::ParticlesLayer},
		Renderer
	},
	sim::{receptors::attract::AttractionReceptor, Simulation, Tick},
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
	fn new(
		simulation: Arc<Mutex<Simulation>>,
		particle_system: Arc<Mutex<ParticleSystem>>
	) -> Self {
		let window = Window::new(APP_NAME, |gl| {
			let mut renderer = Renderer::new(gl);
			renderer.push_layer(|ctx| DotsLayer::new(ctx, simulation));
			renderer.push_layer(|ctx| ParticlesLayer::new(ctx, particle_system));
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
	fn new(
		simulation: Arc<Mutex<Simulation>>,
		particle_system: Arc<Mutex<ParticleSystem>>
	) -> Self {
		let synced_thread = SyncedThread::new(move |recv| {
			for dt in recv {
				{
					let mut sim_lock = simulation.lock().unwrap();
					sim_lock.tick(dt);
				}
				{
					let mut ps_lock = particle_system.lock().unwrap();
					ps_lock.tick(dt);
				}
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
const FRAME_DURATION: Duration = Duration::from_secs_f64(1.0 / TPS);

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
		thread::spawn(move || {
			let mut last_frame = SystemTime::now();
			loop {
				if last_frame.elapsed().unwrap() >= FRAME_DURATION {
					self.tick(last_frame.elapsed().unwrap().as_secs_f64());
					last_frame = SystemTime::now();
				}
			}
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
		let particle_system = Arc::new(Mutex::new(ParticleSystem::new()));
		let simulation = Arc::new(Mutex::new(Self::create_simulation(Arc::clone(
			&particle_system
		))));

		let mut timing_thread = TimingThread::new();
		let window_thread =
			WindowThread::new(Arc::clone(&simulation), Arc::clone(&particle_system));
		let sim_thread = SimThread::new(simulation, particle_system);

		timing_thread.add_handle(sim_thread.sync_handle());
		timing_thread.add_handle(window_thread.sync_handle());

		Self {
			sim_thread,
			window_thread,
			timing_thread
		}
	}

	fn create_simulation(particle_system: Arc<Mutex<ParticleSystem>>) -> Simulation {
		let mut sim = Simulation::new(particle_system);
		sim.add_cell(
			10.0,
			vec3(0.5, 0.5, 0.0),
			vec2(0.0, 0.0),
			vec![Box::new(AttractionReceptor::new(vec3(0.0, 60.0, 50.0)))]
		);
		sim.add_cell(
			3.0,
			vec3(0.2, 0.5, 1.0),
			vec2(500.0, 10.0),
			vec![Box::new(AttractionReceptor::new(vec3(10.0, 0.0, 0.0)))]
		);
		sim.add_cell(
			5.0,
			vec3(0.2, 0.5, 1.0),
			vec2(-200.0, -100.0),
			vec![Box::new(AttractionReceptor::new(vec3(50.0, 0.0, 0.0)))]
		);
		sim
	}

	pub fn start(self) {
		self.timing_thread.start();
		self.sim_thread.start();
		self.window_thread.start_sync();
	}
}
