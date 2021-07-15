// Panel UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};

// Modules
use crate::app::services::auth::AuthService;
use crate::app::state::State;

/// Left Panel
pub struct LeftPanel {
	username: String,
	password: String,
}

impl LeftPanel {
	// Constructor - Initialize Widgets / Components
	pub fn new() -> Self {
		Self {
			username: "".to_string(),
			password: "".to_string(),
		}
	}

	// Update - Draws UI each Frame
	pub fn update(&mut self, ctx: &egui::CtxRef, state: &mut State, _: &mut epi::Frame<'_>) {
		egui::SidePanel::left("side_panel").show(ctx, |ui| {
			if state.is_authorized {
				// Authorized

				// self.auth_menu.update(ctx, state, frame);

				// Header
				ui.heading("Authorized");
				// Text Edit
				ui.horizontal(|ui| {
					ui.label("Write something boss ;): ");
					ui.text_edit_singleline(&mut state.label);
				});

				// Slide & Button
				ui.add(egui::Slider::new(&mut state.value, 0.0..=10.0).text("value"));
				if ui.button("Increment").clicked() {
					state.value += 1.0;
				}
			} else {
				// Not Authorized
				ui.heading("Welcome");
				ui.label("Please login to continue:");
				ui.separator();
				ui.horizontal(|ui| {
					ui.label("Username: ");
					ui.text_edit_singleline(&mut self.username);
				});
				ui.horizontal(|ui| {
					ui.label("Password: ");
					ui.text_edit_singleline(&mut self.password);
				});
				if ui.button("Login").clicked() {
					AuthService::login(&self.username, &self.password, state);
				}
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
