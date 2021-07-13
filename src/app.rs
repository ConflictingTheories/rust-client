// Dependencies
use crate::app::egui::Vec2;
use eframe::{egui, epi};

// Modules
mod components;
mod layout;
mod state;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    state: state::State,
}

// Initialization
impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            state: state::State::new("Hello World!".to_owned(), 2.7),
        }
    }
}

impl epi::App for TemplateApp {
    /// Max window Size
    fn max_size_points(&self) -> Vec2 {
        Vec2::new(1920.0, 1080.0)
    }

    /// Name of App
    fn name(&self) -> &str {
        "EGUI Test"
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
        // Core App Variables
        let Self { state } = self;

        // Menu
        layout::menu::TopMenu::update(ctx, state, frame);
        // Side Panel
        layout::panels::LeftPanel::update(ctx, state, frame);
        // Main Panel
        layout::panels::MainPanel::update(ctx, state, frame);
        // Show Window
        if true {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
