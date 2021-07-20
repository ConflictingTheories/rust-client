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
		egui::CentralPanel::default().show(ctx, |_ui| {
			// ATTN: IMMEDIATE MODE
			if state.is_authorized {
				// Cash Flow Plot area
				self.cash_flow.render(ctx, state, frame);
				// -- State Based Windows
				// Window for Project
				if state.new_proj {
					self.new_project_window.render(ctx, state, frame);
				}
			}
		});
	}
}
