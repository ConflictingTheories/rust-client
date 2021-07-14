// Menu UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};

// Modules
use crate::app::ClientApp;

// Top Menu
pub struct TopMenu {}
impl TopMenu {
	/// Default Menu UI
	pub fn update(ctx: &egui::CtxRef, app: &mut ClientApp, frame: &mut epi::Frame<'_>) {
		let ClientApp {
			state,
			new_state: _,
		} = app;
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				egui::menu::menu(ui, "File", |ui| {
					if ui.button("New Project").clicked() {
						state.new_proj = true;
					}
					if ui.button("Quit").clicked() {
						frame.quit();
					}
				});
			});
		});
	}
}
