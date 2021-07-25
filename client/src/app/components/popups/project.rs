// Panel UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};

// Modules
use crate::app::state::State;
use crate::app::components::Component;

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