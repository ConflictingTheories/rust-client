pub mod animations;
pub mod graph;
pub mod popups;

// Dependencies
use eframe::{egui, epi};

// Modules
pub use crate::app::state::State;

pub trait Component {
	fn render(&mut self, ui: &mut egui::Ui, state: &mut State) -> ();
}