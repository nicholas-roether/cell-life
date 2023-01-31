use app::App;
mod app;
mod render;
mod simulation;
mod window;

fn main() {
	let app = App::new();
	app.start();
}
