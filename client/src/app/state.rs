#![allow(dead_code)]

use crate::app::services::easter_egg::{EasterEgg, Key, Secret, SecretType};

pub struct State {
	pub label: String,
	pub value: f32,
	pub new_proj: bool,
	pub is_authorized: bool,
	pub easter_eggs_found: i32,
	pub easter_eggs: Vec<EasterEgg>,
}

impl State {
	pub fn new(label: String, value: f32) -> Self {
		// Initialize the Secrets ;)
		let mut eggs = Vec::<EasterEgg>::new();
		let mut east = EasterEgg::new("test-egg");
		let mut secret = Secret::new(SecretType::int);
		secret.seti32(100);
		east.set_secret(secret);
		eggs.push(east);

		Self {
			label,
			value,
			new_proj: false,
			is_authorized: false,
			easter_eggs_found: 0,
			easter_eggs: eggs,
		}
	}

	pub fn findEgg(&mut self, egg: &mut EasterEgg, key: &Key) {
		if *egg.unlock(key) {
			self.easter_eggs_found += 1;
		}
	}
}
