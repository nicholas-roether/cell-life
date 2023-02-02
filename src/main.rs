#![cfg_attr(release, windows_subsystem = "windows")]
#![feature(duration_consts_float)]

use app::App;
mod app;
mod render;
mod sim;
mod utils;
mod window;

fn main() {
	let app = App::new();
	app.start();
}
