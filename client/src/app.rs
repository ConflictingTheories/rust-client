// Dependencies
use eframe::{egui, epi};
use egui::Vec2;

// Modules
mod components;
mod layout;
mod services;
mod state;

use crate::app::services::easter_egg::{EasterEgg, Key, Secret, SecretType};
use std::collections::HashMap;

/// Client App - Has State - We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct ClientApp {
    pub state: state::State,
    top_menu: layout::menu::TopMenu,
    side_panel: layout::left::LeftPanel,
    main_panel: layout::main::MainPanel,
}

// Initialization
impl Default for ClientApp {
    fn default() -> Self {
        let mut Eggs = HashMap::<String, EasterEgg>::new();

        for e in vec!["red", "blue", "green", "test-egg"] {
            // Initialize Base Easter Eggs
            match e {
                "red" => {
                    let mut east = EasterEgg::new(e);
                    let mut secret = Secret::new(SecretType::int);
                    secret.seti32(1010);
                    east.set_secret(secret);
                    Eggs.insert(e.to_string(), east);
                }
                "green" => {
                    let mut east = EasterEgg::new(e);
                    let mut secret = Secret::new(SecretType::float);
                    secret.setf32(10.0);
                    east.set_secret(secret);
                    Eggs.insert(e.to_string(), east);
                }
                "blue" => {
                    let mut east = EasterEgg::new(e);
                    let mut secret = Secret::new(SecretType::string);
                    secret.setstr("yolo");
                    east.set_secret(secret);
                    Eggs.insert(e.to_string(), east);
                }
                "test-egg" => {
                    let mut east = EasterEgg::new(e);
                    let mut secret = Secret::new(SecretType::int);
                    secret.seti32(100);
                    east.set_secret(secret);
                    Eggs.insert(e.to_string(), east);
                }
                _ => {}
            }
        }

        Self {
            state: state::State::new("Hello World!".to_owned(), 2.7, Some(Eggs)), 
            #[cfg_attr(feature = "persistence", serde(skip))]   // ignore 
            top_menu: layout::menu::TopMenu::new(),
            #[cfg_attr(feature = "persistence", serde(skip))]   // ignore
            side_panel: layout::left::LeftPanel::new(),
            #[cfg_attr(feature = "persistence", serde(skip))]   // ignore
            main_panel: layout::main::MainPanel::new(),
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
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { state, .. } = self;
        // Core App Variables (state)
        self.top_menu.render(ctx, state, frame);
        self.side_panel.render(ctx, state, frame);
        self.main_panel.render(ctx, state, frame);
    }
}
