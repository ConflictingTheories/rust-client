// Panel UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};

// Modules
use crate::app::state::State;

// Center Panel
pub struct MainPanel {}
impl MainPanel {
	/// Default Menu UI
	pub fn update(ctx: &egui::CtxRef, _: &mut State, _: &mut epi::Frame<'_>) {
		egui::CentralPanel::default().show(ctx, |ui| {
			// The central panel the region left after adding TopPanel's and SidePanel's
			ui.heading("Calliope");
			ui.hyperlink("https://calliope.site");
			egui::warn_if_debug_build(ui);
		});
	}
}

// Left Panel
pub struct LeftPanel {}
impl LeftPanel {
	/// Default Menu UI
	pub fn update(ctx: &egui::CtxRef, state: &mut State, _: &mut epi::Frame<'_>) {
		egui::SidePanel::left("side_panel").show(ctx, |ui| {
			// Header
			ui.heading("Side Panel");
			// Text Edit
			ui.horizontal(|ui| {
				ui.label("Write something: ");
				ui.text_edit_singleline(&mut state.label);
			});

			// Slide & Button
			ui.add(egui::Slider::new(&mut state.value, 0.0..=10.0).text("value"));
			if ui.button("Increment").clicked() {
				state.value += 1.0;
			}

			// Bottom Align
			ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
				ui.add(
					egui::Hyperlink::new("https://github.com/ConflictingTheories")
						.text("//ConflictingTheories"),
				);
			});
		});
	}
}