// Menu UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};

// Modules
use crate::app::state::State;

// Top Menu
pub struct TopMenu {}
impl TopMenu {
	pub fn new() -> Self {
		Self {}
	}
	/// Default Menu UI
	pub fn render(&mut self, ctx: &egui::CtxRef, state: &mut State, _frame: &mut epi::Frame<'_>) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				egui::menu::menu(ui, "File", |ui| {
					if state.is_authorized {
						if ui.button("New Project").clicked() {
							state.new_proj = true;
						}
						if ui.button("Logout").clicked() {
							state.is_authorized = false;
						}
					} else {
						if ui.button("Login").clicked() {
							state.is_authorized = false;
						}
					}
				});
			});
		});
	}
}
