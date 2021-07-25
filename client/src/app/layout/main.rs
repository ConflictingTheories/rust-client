// Panel UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};

// Modules
use crate::app::components::{graph, popups};
use crate::app::state::State;

/// Center Panel
pub struct MainPanel {
	new_project_window: popups::project::CreateProjectPopup,
	cash_flow: graph::CashFlowPlot,
}

// State-Enabled Components/ Widgets
impl MainPanel {
	pub fn new() -> Self {
		Self {
			new_project_window: popups::project::CreateProjectPopup::new(),
			cash_flow: graph::CashFlowPlot::new("cash_flow"),
		}
	}

	// Update - Draws UI each Frame
	pub fn render(&mut self, ctx: &egui::CtxRef, state: &mut State, frame: &mut epi::Frame<'_>) {
		// ATTN: IMMEDIATE MODE
		egui::CentralPanel::default()
			.frame(egui::containers::Frame::dark_canvas(&ctx.style()))
			.show(ctx, |ui| {
				// Dark Background
				match state.is_authorized {
					// Logged in
					(true) => {
						// Cash Flow Plot area
						self.cash_flow.render(ctx, state, frame);
						// -- State Based Windows
						// Window for Project
						if state.new_proj {
							self.new_project_window.render(ctx, state, frame);
						}
					}
					// Not Authorized
					(false) => {
						// Signup / Registration
						ui.label("Welcome to the Rusty Client!");
					}
				}
			});
	}
}
