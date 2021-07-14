// Panel UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};

// Modules
use crate::app::ClientApp;

// Center Panel
pub struct CreateProjectWindow {}
impl CreateProjectWindow {
	/// Default Menu UI
	pub fn update(ctx: &egui::CtxRef, app: &mut ClientApp, _: &mut epi::Frame<'_>) {
		let ClientApp { state, new_state } = app;
		egui::Window::new("Project Name").show(ctx, |ui| {
			ui.label("What would you like you call your project?");
			ui.text_edit_singleline(&mut new_state.label);
			ui.separator();
			// Save
			if ui.button("Create").clicked() {
				state.label = new_state.label.to_string();
				new_state.label = "".to_string(); // clear
				state.new_proj = false;
			}
			if ui.button("Cancel").clicked() {
				state.new_proj = false;
			}
		});
	}
}
