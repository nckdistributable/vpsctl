use crate::Config;
use core::panic;
use std::fs;

pub fn init(config: Config) {
    if !config.vpsconfig_dir_path().exists() {
        match fs::create_dir_all(config.vpsconfig_dir_path()) {
            Ok(()) => println!("Directory for config created or alredy existed."),
            Err(err) => panic!("Error when creating directory {err}"),
        };
    }
    match fs::write(config.vpsconfig_full_path(), "") {
        Err(err) => panic!("Error on creatin config: {err}"),
        Ok(()) => println!("Config succecefuly created."),
    };
}
