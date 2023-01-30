use app::App;
mod app;
mod render;
mod window;

fn main() {
	let app = App::new();
	app.start();
}
