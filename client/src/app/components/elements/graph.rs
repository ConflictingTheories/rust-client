// Menu UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};
use egui::Vec2;
use std::vec;

// Modules
use crate::app::state::State;

// Cash Flow Plot Graph
pub struct CashFlowPlot {
	label: String,
	x: f32,
	y: f32,
	series: vec::Vec<Vec2>,
	x_axis: String,
	y_axis: String,
}

impl CashFlowPlot {
	/// Initialize Empty Plot
	pub fn new(id: &str) -> Self {
		Self {
			label: id.to_string(),
			x: 1200.0,
			y: 720.0,
			series: vec::Vec::new(),
			x_axis: "".to_string(),
			y_axis: "".to_string(),
		}
	}

	/// Set Series for plotting
	pub fn set_series(&mut self, series: vec::Vec<Vec2>) {
		self.series = series;
	}

	/// Draws UI
	pub fn update(&mut self, ctx: &egui::CtxRef, _state: &mut State, _frame: &mut epi::Frame<'_>) {
		// Chart (Align and Snap to Parent)

		// Settings Area (draggable)
		egui::Area::new(&self.label).show(ctx, |ui| {
			ui.heading("Calliope");
			ui.hyperlink("https://calliope.site");
			egui::warn_if_debug_build(ui);
			ui.label("This is going to be a graph chart - Cash Flow Baby! ;)");
			ui.separator();
		});
	}
}
