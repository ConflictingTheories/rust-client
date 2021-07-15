use crate::app::state::State;

pub struct Auth {}

impl Auth {
	pub fn login(_username: &String, _password: &String, state: &mut State) {
		// Perform Network Call to Server and Login

		// for now return response mock success
		state.is_authorized = true;
	}
}
