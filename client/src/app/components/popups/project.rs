// Panel UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};

// Modules
use crate::app::components::Component;
use crate::app::services::easter_egg::{Key, Secret, SecretType};
use crate::app::state::State;

// State
pub struct CreateProjectPopup {
	label: String,
}

impl CreateProjectPopup {
	pub fn new() -> Self {
		Self {
			label: "".to_string(),
		}
	}
}

impl Component for CreateProjectPopup {
	/// Default Menu UI
	fn render(&mut self, ui: &mut egui::Ui, state: &mut State) {
		egui::Window::new("Project Name").show(ui.ctx(), |ui| {
			ui.label("What would you like you call your project?");
			ui.text_edit_singleline(&mut self.label);
			ui.separator();
			// Save
			if ui.button("Create").clicked() {
				// Easter Egg -- Check Name
				if (&self.label.to_string() == "hello") {
					for (key, e) in &mut state.easter_eggs {
						match key.as_ref() {
							"test-egg" => {
								let mut sct = Secret::new(SecretType::int);
								sct.seti32(100);
								let mut key = Key::new();
								key.encode(&sct);
								e.unlock(&key);
							}
							_ => {}
						}
					}
				}

				// Change Text
				state.label = self.label.to_string();
				self.label.clear();
				state.new_proj = false;
			}
			if ui.button("Cancel").clicked() {
				state.new_proj = false;
			}
		});
	}
}
