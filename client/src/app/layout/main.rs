// Panel UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};
use egui::{containers::*, *};

// Modules
use crate::app::components::{popups, animations, graph, Component};
use crate::app::state::State;

/// Center Panel
pub struct MainPanel {
	new_project_window: popups::project::CreateProjectPopup,
	cash_flow: graph::CashFlowPlot,
	strings: animations::StringsAnimation
}

// State-Enabled Components/ Widgets
impl MainPanel {
	pub fn new() -> Self {
		Self {
			new_project_window: popups::project::CreateProjectPopup::new(),
			cash_flow: graph::CashFlowPlot::new("cash_flow"),
			strings: animations::StringsAnimation::new("cash_flow"),
		}
	}

	// Update - Draws UI each Frame
	pub fn render(&mut self, ctx: &egui::CtxRef, state: &mut State, frame: &mut epi::Frame<'_>) {
		// ATTN: IMMEDIATE MODE
		egui::CentralPanel::default()
			.frame(Frame::dark_canvas(&ctx.style()))
			.show(ctx, |ui| {
				// Dark Background
				match state.is_authorized {
					// Logged in
					(true) => {
						// Cash Flow Plot area
						self.cash_flow.render(ui, state);
						// -- State Based Windows
						// Window for Project

						// TODO - Convert this into a more Flexible State machine driven Enum
						//
						// -- Have a Expandable way of managing window + Popup types
						//
						// -- Perhaps some form of Popup Event Redux kind of thing.
						//
						// -- Not as complex as redux saga though - something simpler.

						if state.new_proj {
							self.new_project_window.render(ui, state);
						}
					}
					// Not Authorized
					(false) => {
						ui.label("Welcome to the Rusty Client!");
						self.strings.render(ui, state);
					}
				}
			});
	}
}
