// Dependencies
use crate::app::egui::Vec2;
use eframe::{egui, epi};

// Modules
mod components;
mod layout;
mod state;

/// Client App - Has State - We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct ClientApp {
    state: state::State,
    new_state: state::State,
}

// Initialization
impl Default for ClientApp {
    fn default() -> Self {
        Self {
            state: state::State::new("Hello World!".to_owned(), 2.7),

            #[cfg_attr(feature = "persistence", serde(skip))]   // Dont Save Temp State
            new_state: state::State::new("".to_owned(), 0.0),
        }
    }
}

impl epi::App for ClientApp {
    /// Max window Size
    fn max_size_points(&self) -> Vec2 {
        Vec2::new(1920.0, 1080.0)
    }

    /// Name of App
    fn name(&self) -> &str {
        "Client App"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        *self = epi::get_value(storage.unwrap(), epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        // Core App Variables (state)
        let Self { state, new_state } = self;
        // Menu
        layout::menu::TopMenu::update(ctx, state, frame);
        // Side Panel
        layout::panels::LeftPanel::update(ctx, state, frame);
        // Main Panel
        layout::panels::MainPanel::update(ctx, state, frame);
        // Show Window
        // Window for Project
        if state.new_proj {
            egui::Window::new("Project Name").show(ctx, |ui| {
                ui.label("What would you like you call your project?");
                ui.text_edit_singleline(&mut new_state.label);
                ui.separator();
                // Save
                if ui.button("Create").clicked() {
                    state.label = new_state.label.to_string();
                    new_state.label = "".to_string(); // clear
                    state.new_proj = false;
                }
                if ui.button("Cancel").clicked() {
                    state.new_proj = false;
                }
            });
        }
    }
}
