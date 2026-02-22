// app

use native_windows_gui as nwg;

use std::fmt::Display;

use strum::IntoEnumIterator;
use nwg::{Event, FileDialogAction, MessageParams, MessageButtons, MessageIcons};

use crate::gunmacro::{GunMacro, Item};

#[derive(Default)]
pub struct App {
	title: String,
	window: nwg::Window,
	list_items: nwg::ListBox<Item>,
	list_sel: nwg::ListBox<Item>,
	btn_add: nwg::Button,
	btn_rmv: nwg::Button,
	btn_gen: nwg::Button,
	btn_debug: nwg::Button,
	dropdown_format: nwg::ComboBox<MacroFormat>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter)]
pub enum MacroFormat {
	#[default] Ahk,
	Plaintext,
}

impl MacroFormat {
	fn filters(&self) -> &'static str {
		match self {
			Self::Ahk => "AutoHotkey (*.ahk)",
			Self::Plaintext => "Text File (*.txt)",
		}
	}
}

impl Display for MacroFormat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Self::Ahk => "AutoHotkey",
			Self::Plaintext => "Plaintext",
		})
	}
}

impl App {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn build_ui(&mut self) {
		if cfg!(debug_assertions) {
			self.title = format!("ice's gunmacro maker v{} [DEBUG]", crate::VERSION);
		} else {
			self.title = format!("ice's gunmacro maker v{}", crate::VERSION);
		}

		// init window
		nwg::Window::builder()
			.size((420, 320))
			.position((300, 300))
			.title(&self.title)
			.build(&mut self.window)
			.unwrap();

		// init items list
		nwg::ListBox::builder()
			.parent(&self.window)
			.size((180, 200))
			.position((20, 20))
			.build(&mut self.list_items)
			.unwrap();
		for item in Item::iter() {
			self.list_items.insert(usize::MAX, item);
		}

		// init selected items list
		nwg::ListBox::builder()
			.parent(&self.window)
			.size((180, 200))
			.position((220, 20))
			.build(&mut self.list_sel)
			.unwrap();

		// init btn add
		nwg::Button::builder()
			.text("Add")
			.parent(&self.window)
			.size((88, 30))
			.position((20, 220))
			.build(&mut self.btn_add)
			.unwrap();

		// init btn remove
		nwg::Button::builder()
			.text("Remove")
			.parent(&self.window)
			.size((88, 30))
			.position((112, 220))
			.build(&mut self.btn_rmv)
			.unwrap();

		// init btn generate
		nwg::Button::builder()
			.parent(&self.window)
			.text("Generate")
			.position((220, 220))
			.size((180, 30))
			.build(&mut self.btn_gen)
			.unwrap();

		// init btn debug
		nwg::Button::builder()
			.parent(&self.window)
			.text("Debug")
			.position((220, 260))
			.size((180, 30))
			.build(&mut self.btn_debug)
			.unwrap();

		// init format dropdown
		nwg::ComboBox::builder()
			.parent(&self.window)
			.position((20, 260))
			.size((180, 30))
			.build(&mut self.dropdown_format)
			.unwrap();
		for format in MacroFormat::iter() {
			self.dropdown_format.insert(usize::MAX, format);
		}
	}

	pub fn run(self) {
		let h_window = self.window.handle;
		let h_btn_add = self.btn_add.handle;
		let h_btn_rmv = self.btn_rmv.handle;
		let h_btn_gen = self.btn_gen.handle;
		let h_btn_debug = self.btn_debug.handle;

		nwg::full_bind_event_handler(&self.window.handle, move |event, _, handle| {
			match event {
				// close window
				Event::OnWindowClose if handle == h_window => {
					println!("Window closed");
					nwg::stop_thread_dispatch();
				}

				// add button
				Event::OnButtonClick if handle == h_btn_add => {
					if let Some(sel) = self.list_items.selection() {
						let item = self.list_items.remove(sel);
						self.list_sel.insert(usize::MAX, item);
					}
				}

				// rmv button
				Event::OnButtonClick if handle == h_btn_rmv => {
					if let Some(sel) = self.list_sel.selection() {
						let item = self.list_sel.remove(sel);
						let idx = self.list_items.collection().partition_point(|&x| x < item);
						self.list_items.insert(idx, item);
					}
				}

				// gen button
				Event::OnButtonClick if handle == h_btn_gen => {
					let build = self.list_sel.collection();
					if build.len() == 0 {
						nwg::message(&MessageParams {
							title: "Error",
							content: "Must select at least one item",
							icons: MessageIcons::Error,
							buttons: MessageButtons::Ok,
						});
						return;
					}

					let fmt = match self.dropdown_format.selection() {
						Some(idx) => self.dropdown_format.collection()[idx],
						None => {
							nwg::message(&MessageParams {
								title: "Error",
								content: "Must select a file format",
								icons: MessageIcons::Error,
								buttons: MessageButtons::Ok,
							});
							return;
						}
					};

					let gm = GunMacro::from_items(&build);

					let mut dialog = nwg::FileDialog::default();
					nwg::FileDialog::builder()
						.title("Save Macro")
						.action(FileDialogAction::Save)
						.filters(fmt.filters())
						.build(&mut dialog)
						.unwrap();

					if dialog.run(Some(h_window)) {
						if let Ok(path) = dialog.get_selected_item() {
							let mut path = std::path::PathBuf::from(path);
							
							if path.extension().is_none() {
								match fmt {
									MacroFormat::Ahk => path.set_extension("ahk"),
									MacroFormat::Plaintext => path.set_extension("txt"),
								};
							}

							let content = match fmt {
								MacroFormat::Ahk => gm.to_ahk_script(),
								MacroFormat::Plaintext => gm.to_plaintext(),
							};

							match std::fs::write(&path, content) {
								Ok(_) => nwg::message(&MessageParams {
									title: "Success",
									content: &format!("Saved macro to {path:?}"),
									icons: MessageIcons::Info,
									buttons: MessageButtons::Ok,
								}),

								Err(e) => nwg::message(&MessageParams {
									title: "Error",
									content: &format!("Failed to save macro\n\nError:{e}"),
									icons: MessageIcons::Error,
									buttons: MessageButtons::Ok,
								}),
							};
						}
					}
				}

				// debug click
				Event::OnButtonClick if handle == h_btn_debug => {
					let build = self.list_sel.collection();
					if build.len() == 0 {
						nwg::message(&MessageParams {
							title: "Error",
							content: "Must select at least one item",
							icons: MessageIcons::Error,
							buttons: MessageButtons::Ok,
						});
						return;
					}

					let gm = GunMacro::from_items(&build);
					nwg::message(&MessageParams {
						title: "Debug",
						content: &gm.to_plaintext(),
						icons: MessageIcons::Info,
						buttons: MessageButtons::Ok,
					});
				}
				
				// ignore other
				_ => {}
			}
		});

		nwg::dispatch_thread_events();
	}
}