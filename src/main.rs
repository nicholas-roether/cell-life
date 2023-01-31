use app::App;
mod app;
mod model;
mod render;
mod window;

fn main() {
	let app = App::new();
	app.start();
}
