#![cfg_attr(release, windows_subsystem = "windows")]
#![feature(duration_consts_float)]

use app::App;
mod app;
mod ecs;
mod particles;
mod rand;
mod render;
mod sim;
mod window;

fn main() {
	let app = App::new();
	app.start();
}
