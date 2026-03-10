use core::panic;
use std::env;
use std::fs;
use std::path;
use std::path::PathBuf;
use env::current_dir;
use crate::Config;
const CONFIG_FILE: &str = ".config/vpsctl/config.toml";
const CONFIG_FILE_DEBUG: &str = "config.toml";

pub fn init(config: Config) {
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!")
    };

    let mut path_to_config: PathBuf = home_dir.to_path_buf();
    let profile_name = config.profile();

    match profile_name {
        "dev" => {
            path_to_config = env::current_exe()
                .unwrap()
                .parent().unwrap()
                .parent().unwrap()
                .parent().unwrap()
                .join("config.toml");
            println!("{path_to_config:?}");
        }
        _ => {
            path_to_config = home_dir.join(CONFIG_FILE_DEBUG);
        }
    }

    match fs::exists(&path_to_config) {
        Ok(true) => println!("Config already initialized"),
        Ok(false) => init_config(&path_to_config),
        Err(err) => panic!("Error : {err}"),
    };
}

fn init_config(config_path: &PathBuf) {
    match config_path.parent() {
        Some(parent) => {
            match fs::create_dir_all(parent) {
                Ok(()) => println!("Directory for config created or alredy existed."),
                Err(err) => panic!("Error when creating directory {err}")
            };
        }
        None => {}
    }
    match fs::write(config_path, "") {
        Err(err) => panic!("Error on creatin config: {err}"),
        Ok(()) => println!("Config succecefuly created."),
    };
}

