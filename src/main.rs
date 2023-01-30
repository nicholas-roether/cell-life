use window::Window;

// use app::App;
// mod app;
// mod renderer;
mod window;

fn main() {
	let window = Window::new("Cell Life");
	window.run();
}
