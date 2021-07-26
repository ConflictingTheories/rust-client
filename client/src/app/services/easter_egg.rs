pub enum SecretType {
	int,
	float,
	string,
	unset,
}
// Secret

pub struct Secret {
	pub s_type: SecretType,
	i_32: i32,
	f_32: f32,
	c_str: String,
}

impl Secret {
	pub fn new(s_type: SecretType) -> Self {
		Self {
			s_type: s_type,
			i_32: 0,
			f_32: 0.0,
			c_str: "".to_string(),
		}
	}

	pub fn get_type(&self) -> SecretType {
		match self.s_type {
			SecretType::int => {
				return SecretType::int;
			}
			SecretType::float => {
				return SecretType::float;
			}
			SecretType::string => return SecretType::string,
			SecretType::unset => {
				return SecretType::unset;
			}
		}
	}

	// Number
	pub fn seti32(&mut self, i: i32) {
		self.i_32 = i;
	}
	pub fn geti32(&self) -> &i32 {
		return &self.i_32;
	}
	// Float
	pub fn getf32(&self) -> &f32 {
		return &self.f_32;
	}
	pub fn setf32(&mut self, i: f32) {
		self.f_32 = i;
	}
	// String
	pub fn getstr(&self) -> String {
		return self.c_str.to_string();
	}
	pub fn setstr(&mut self, i: &str) {
		self.c_str = i.to_string();
	}

	pub fn check(&self, key: &Key) -> bool {
		match key.get_type() {
			SecretType::int => {
				return key.geti32() == &self.i_32;
			}
			SecretType::float => {
				return key.getf32() == &self.f_32;
			}
			SecretType::string => {
				return key.getstr() == self.c_str.to_string();
			}
			SecretType::unset => {
				return false;
			}
		}
	}
}

// Key
pub struct Key {
	pub s_type: SecretType,
	i_32: i32,
	f_32: f32,
	c_str: String,
}

impl Key {
	pub fn new() -> Self {
		Self {
			s_type: SecretType::unset,
			i_32: 0,
			f_32: 0.0,
			c_str: "".to_string(),
		}
	}
	// Type
	pub fn get_type(&self) -> &SecretType {
		return &self.s_type;
	}
	// Number
	pub fn geti32(&self) -> &i32 {
		return &self.i_32;
	}
	// Float
	pub fn getf32(&self) -> &f32 {
		return &self.f_32;
	}
	// String
	pub fn getstr(&self) -> String {
		return self.c_str.to_string();
	}
	// Encode based on Secret
	pub fn encode(&mut self, secret: &Secret) {
		self.s_type = secret.get_type();
		self.i_32 = *secret.geti32();
		self.f_32 = *secret.getf32();
		self.c_str = (*secret.getstr()).to_string();
	}
}

// Easter Egg
pub struct EasterEgg {
	pub label: String,
	discovered: bool,
	secret: Secret,
	key: Key,
}

impl EasterEgg {
	pub fn new(label: &str) -> Self {
		Self {
			label: label.to_string(),
			discovered: false,
			secret: Secret::new(SecretType::unset),
			key: Key::new(),
		}
	}

	// Set New Secret and Lock
	pub fn set_secret(&mut self, secret: Secret) {
		self.secret = secret;
		self.lock();
	}

	// has it been unlocked?
	pub fn is_found(&self) -> bool {
		return self.discovered;
	}

	// Lock it
	pub fn lock(&mut self) {
		self.key.encode(&self.secret);
		self.discovered = false;
	}

	// Unlock it
	pub fn unlock(&mut self, key: &Key) -> &bool {
		if self.secret.check(key) {
			self.discovered = true;
		}
		return &self.discovered;
	}
}
