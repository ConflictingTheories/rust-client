// use serde::{Deserialize, Serialize};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::{Request, RequestInit, RequestMode, Response};
use web_sys::console;

pub struct Api {}

impl Api {
	pub fn get(_url: &str, _params: &RequestParams) {
		// Perform Network Call to Server and Login
	}

	pub fn post(url: &str, params: &RequestParams) {
		// Perform Network Call to Server and Login
		// console::log_1(json::parse(&params.to_json()).unwrap());
	}

	// put

	// delete

	// patch
}

pub struct Response {}

impl Response {}

pub struct RequestParams {
	params: json::JsonValue,
}

impl RequestParams {
	pub fn new() -> Self {
		Self {
			params: json::JsonValue::new_object(),
		}
	}

	pub fn add(&mut self, key: &str, value: &str) {
		self.params[key] = value.into();
	}

	pub fn set(&mut self, value: &str) {
		self.params = json::parse(value).unwrap();
	}

	pub fn to_json(&mut self) -> String {
		self.params.dump()
	}
}
