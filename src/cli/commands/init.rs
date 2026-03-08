use core::panic;
use std::env;
use std::fs;
use std::path;
use std::path::PathBuf;

pub fn init() {
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => panic!("Impossible to get your home dir!")
    };

    let path_to_config = path::PathBuf::from(home_dir).join(".config/vpsctl/config.yml");
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
                Ok(()) => println!("Directory for config created "),
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

