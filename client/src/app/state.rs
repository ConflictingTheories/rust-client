#![allow(dead_code)]

pub struct State {
	pub label: String,
	pub value: f32,
}

impl State {
	pub fn new(label: String, value: f32) -> Self {
		Self { label, value }
	}

	pub fn set_label(&mut self, label: String) {
		self.label = label;
	}

	pub fn set_value(&mut self, value: f32) {
		self.value = value;
	}
}