// gunmacro

mod class;
mod input;
mod item;

pub use class::Class;
pub use item::Item;
pub use input::Input;

use crate::utils;

#[derive(Debug)]
pub struct GunMacroBuilder {
	cursor: i32,
	class: Class,
	inputs: Vec<(Input, u32)>,
}

#[derive(Debug)]
pub struct GunMacro {
	pub inputs: Vec<(Input, u32)>,
}

impl GunMacroBuilder {
	pub fn new() -> Self {
		Self {
			cursor: 0,
			class: Class::Gun,
			inputs: vec![
				(Input::Backslash, 1),
				(Input::Left, 4),
				(Input::Right, 2),
			]
		}
	}

	fn add_input(&mut self, input: Input) {
		self.add_input_n(input, 1);
	}

	fn add_input_n(&mut self, input: Input, n: u32) {
		if n > 0 {
			self.inputs.push((input, n))
		}
	}

	pub fn move_cursor(&mut self, d: i32) {
		self.cursor += d;
		if d < 0 {
			self.add_input_n(Input::Left, -d as u32);
		} else if d > 0 {
			self.add_input_n(Input::Right, d as u32);
		}
	}
	
	pub fn move_cursor_to(&mut self, c: i32) {
		self.move_cursor(c - self.cursor);
	}

	pub fn select_class(&mut self, class: Class) {
		self.move_cursor_to(0);
		match class { // pick the menu
			Class::Gun => self.add_input(Input::Up),
			Class::Explosive => {},
			Class::Misc => self.add_input(Input::Down),
		}
		self.add_input(Input::Enter);
		match class { // align the cursor to the misc menu button
			Class::Gun => self.add_input_n(Input::Down, 2),
			Class::Explosive => self.add_input(Input::Down),
			Class::Misc => {},
		}
	}

	pub fn grab_item(&mut self, item: Item) {
		if self.class != item.class() {
			self.select_class(item.class());
		}

		self.move_cursor_to(item.order()); // move to the item
		self.add_input(Input::Enter); // equip the item
	}

	pub fn finish(mut self) -> GunMacro {
		self.select_class(Class::Gun);
		self.add_input(Input::Backslash);
		utils::dedup_near(&mut self.inputs);

		GunMacro {
			inputs: self.inputs
		}
	} 
}

impl GunMacro {
	pub fn from_items(items: &[Item]) -> Self {
		let mut gm_builder = GunMacroBuilder::new();
		for &item in items {
			gm_builder.grab_item(item);
		}
		gm_builder.finish()
	}

	pub fn to_ahk_script(&self) -> String {
		let mut s = String::new();
		s.push_str("Keybind::{\n");
		for &(input, n) in &self.inputs {
			let term = match input {
				Input::Backslash => "\\".into(),
				other => format!("{:?}", other),
			};
			let action = match n {
				1 => term,
				_ => format!("{} {}", term, n),
			};
			s.push('\t');
			s += &format!(r#"Send("{{{}}}")"#, action);
			s.push('\n');
		}
		s.push('}');
		s
	}

	pub fn to_plaintext(&self) -> String {
		let mut s = String::new();
		for &(input, n) in &self.inputs {
			s.push_str(&format!("{input:?}"));
			if n > 1 {
				s.push_str(&format!(" x{n}"));
			}
			s.push('\n');
		}
		s
	}
}