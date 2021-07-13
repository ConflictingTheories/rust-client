// Menu UI
#![allow(dead_code)]

use crate::app::state::State;
use eframe::{egui, epi};

// Top Menu
pub struct TopMenu {}
impl TopMenu {
	/// Default Menu UI
	pub fn update(ctx: &egui::CtxRef, _: &mut State, frame: &mut epi::Frame<'_>) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			egui::menu::bar(ui, |ui| {
				egui::menu::menu(ui, "File", |ui| {
					if ui.button("Quit").clicked() {
						frame.quit();
					}
				});
			});
		});
	}
}
