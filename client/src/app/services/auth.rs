#![cfg_attr(not(debug_assertions), deny(warnings))]

// use serde::{Deserialize, Serialize};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::app::state::State;
use crate::app::services::api::{Api, RequestParams};

pub struct AuthService {}

impl AuthService {
	pub fn login(username: &String, password: &String, state: &mut State) {
		// Credentials
		let mut params: RequestParams = RequestParams::new();
		params.add("username", username);
		params.add("password", password);

		// Login
		Api::post("/login", &params);

		// for now return response mock success
		state.is_authorized = true;
	}
}
