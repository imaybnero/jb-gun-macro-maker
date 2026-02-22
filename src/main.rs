// main

#![cfg_attr(debug_assertions, allow(unused))]
#![windows_subsystem = "windows"]

mod app;
mod gunmacro;
mod utils;

use native_windows_gui as nwg;
use crate::app::App;

pub const CHANGELOG: &str = include_str!("../CHANGELOG");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
	nwg::init().expect("Failed to init NWG");
	nwg::Font::set_global_family("Segoe UI").expect("Failed to set font");
	
	let mut app = App::new();
	app.build_ui();
	app.run();
}