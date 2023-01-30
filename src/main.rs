use app::App;
mod app;
mod renderer;
mod window;

fn main() {
	let app = App::new();
	app.start();
}
