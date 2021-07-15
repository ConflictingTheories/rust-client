#![allow(dead_code)]

pub struct State {
	pub label: String,
	pub value: f32,
	pub new_proj: bool,
	pub is_authorized: bool,
}

impl State {
	pub fn new(label: String, value: f32) -> Self {
		Self {
			label,
			value,
			new_proj: false,
			is_authorized: false,
		}
	}
}
