// main

#![allow(unused)]

mod app;
mod gunmacro;
mod utils;

use native_windows_gui as nwg;

use std::cell::RefCell;
use std::rc::Rc;
use nwg::{NativeUi, Event, MessageParams, MessageButtons, MessageIcons, MessageChoice};
use strum::IntoEnumIterator;

use gunmacro::{
	Class, Input, Item,
	GunMacro, GunMacroBuilder,
};

use crate::app::App;

const CHANGELOG: &str = include_str!("../CHANGELOG");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
	nwg::init().expect("Failed to init NWG");
	nwg::Font::set_global_family("Segoe UI").expect("Failed to set font");
	
	let mut app = App::new();
	app.build_ui();
	app.run();
}