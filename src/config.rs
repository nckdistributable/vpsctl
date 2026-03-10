use std::env;

pub struct Config {
    profile: String,
}

impl Config {
    pub fn init() -> Self {
        Self { profile: env::var("PROFILE").unwrap().to_string() }
    }

    pub fn profile(&self) -> &str {
        &self.profile
    }
}
