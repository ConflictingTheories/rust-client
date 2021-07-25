// Panel UI
#![allow(dead_code)]

// Dependencies
use eframe::{egui, epi};
use egui::{containers::*, *};

// Modules
use crate::app::components::Component;
use crate::app::state::State;

// Animated Strings
pub struct StringsAnimation {
	label: String,
}

impl StringsAnimation {
	pub fn new(label: &str) -> Self {
		Self {
			label: label.to_string(),
		}
	}
}

impl Component for StringsAnimation {
	/// Default Menu UI
	fn render(&mut self, ui: &mut egui::Ui, state: &mut State) {
		ui.ctx().request_repaint();
		let time = ui.input().time;

		let desired_size: Vec2 = ui.available_width() * vec2(1.0, 0.35);
		let (_id, rect) = ui.allocate_space(desired_size);

		let to_screen =
			emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);

		let mut shapes = vec![];

		for &mode in &[2, 3, 5] {
			let mode = mode as f32;
			let n = 120;
			let speed = 1.5;

			let points: Vec<Pos2> = (0..=n)
				.map(|i| {
					let t = i as f32 / (n as f32);
					let amp = (time as f32 * speed * mode).sin() / mode;
					let y = amp * (t * std::f32::consts::TAU / 2.0 * mode).sin();
					to_screen * pos2(t, y)
				})
				.collect();

			let thickness = 10.0 / mode;
			shapes.push(epaint::Shape::line(
				points,
				Stroke::new(thickness, Color32::from_additive_luminance(196)),
			));
		}

		ui.painter().extend(shapes);
	}
}
