#![allow(dead_code)]
use crate::app::services::easter_egg::{EasterEgg, Key, Secret, SecretType};
use std::collections::HashMap;

pub struct State {
	pub label: String,
	pub value: f32,
	pub new_proj: bool,
	pub is_authorized: bool,
	pub easter_eggs_found: i32,
	pub easter_eggs: HashMap<String, EasterEgg>,
}

impl State {
	pub fn new(label: String, value: f32, eggs: Option<HashMap<String, EasterEgg>>) -> Self {
		match eggs {
			Some(eggs) => {
				return Self {
					label,
					value,
					new_proj: false,
					is_authorized: false,
					easter_eggs_found: 0,
					easter_eggs: eggs,
				}
			}
			None => {
				return Self {
					label,
					value,
					new_proj: false,
					is_authorized: false,
					easter_eggs_found: 0,
					easter_eggs: HashMap::<String, EasterEgg>::new(),
				}
			}
		}
	}

	/// Get Easter Egg
	pub fn getEasterEgg(&mut self, key: &str) -> Option<&mut EasterEgg> {
		return self.easter_eggs.get_mut(key);
	}

	/// Set Easter Egg
	pub fn setEasterEgg(&mut self, key: &str, egg: EasterEgg) {
		self.easter_eggs.insert(key.to_string(), egg);
	}

	/// Unlock an Easter Egg (true on success)
	pub fn unlockEasterEgg(&mut self, key: &str, unlockKey: &Key) -> bool {
		match self.getEasterEgg(key) {
			Some(egg) => {
				if *egg.unlock(unlockKey) {
					self.easter_eggs_found += 1;
					return true;
				}
				return false;
			}
			None => {
				// do nothing fake egg
				return false;
			}
		}
	}
}
