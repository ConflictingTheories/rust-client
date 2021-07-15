// Panel UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};

// Modules
use crate::app::components::{windows, elements};
use crate::app::state::State;

/// Center Panel
pub struct MainPanel {
	new_project_window: windows::CreateProjectWindow,
	cash_flow: elements::graph::CashFlowPlot,
}
impl MainPanel {
	// Constructor - Initialize Widgets / Components
	pub fn new() -> Self {
		Self {
			new_project_window: windows::CreateProjectWindow::new(),
			cash_flow: elements::graph::CashFlowPlot::new("cash_flow"),
		}
	}

	// Update - Draws UI each Frame
	pub fn update(&mut self, ctx: &egui::CtxRef, state: &mut State, frame: &mut epi::Frame<'_>) {
		egui::CentralPanel::default().show(ctx, |_ui| {
			
			// Cash Flow Plot area
			self.cash_flow.update(ctx, state, frame);
			
			// -- State Based Windows
			
			// Window for Project
			if state.new_proj {
				self.new_project_window.update(ctx, state, frame);
			}
		
		});
	}
}

/// Left Panel
pub struct LeftPanel {}
impl LeftPanel {
	// Constructor - Initialize Widgets / Components
	pub fn new() -> Self {
		Self {}
	}

	// Update - Draws UI each Frame
	pub fn update(&mut self, ctx: &egui::CtxRef, state: &mut State, _: &mut epi::Frame<'_>) {
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
