use std::env;
use std::path::PathBuf;

pub struct Config {
    profile: String,
    vpsconfig_file_name: String,
    vpsconfig_directory_path: PathBuf,
}

impl Config {
    pub fn init() -> Self {
        let profile = env::var("PROFILE").unwrap().to_string();
        let vpsconfig_file_name = env::var("VPS_CONFIG_FILE_NAME").unwrap().to_string();
        let vpsconfig_directory_path = if profile.eq("dev") {
            env::current_dir().unwrap().join("resources/")
        } else {
            env::home_dir().unwrap().join(".config/")
        };
        Self {
            profile,
            vpsconfig_file_name,
            vpsconfig_directory_path
        }

    }

    pub fn profile(&self) -> &str {
        if self.profile.is_empty() {
            panic!("config wasn't initialized")
        }
        &self.profile
    }

    pub fn vpsconfig_file_name(&self) -> &str {
        if self.vpsconfig_file_name.is_empty() {
            panic!("config wasn't initialized")
        }
        &self.vpsconfig_file_name
    }

    pub fn vpsconfig_dir_path(&self) -> &PathBuf {
        &self.vpsconfig_directory_path
    }

    pub fn vpsconfig_full_path(&self) -> PathBuf {
        self.vpsconfig_directory_path.join(&self.vpsconfig_file_name)
    }

}
