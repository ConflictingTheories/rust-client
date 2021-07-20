// Panel UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};

// Modules
use crate::app::state::State;

// Center Panel
pub struct CreateProjectPopup {
	label: String,
}
impl CreateProjectPopup {
	pub fn new() -> Self {
		Self {
			label: "".to_string(),
		}
	}
	/// Default Menu UI
	pub fn render(&mut self, ctx: &egui::CtxRef, state: &mut State, _: &mut epi::Frame<'_>) {
		egui::Window::new("Project Name").show(ctx, |ui| {
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
