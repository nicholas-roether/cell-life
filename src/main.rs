#![feature(duration_consts_float)]

use app::App;
mod app;
mod render;
mod sim;
mod window;

fn main() {
	let app = App::new();
	app.start();
}
