mod cli;
use std::env;
use dotenvy::dotenv;
mod vps_config;
mod config;
use config::Config;

fn main() {
    dotenv().ok();
    let config = Config::init();

    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|val| String::as_str(val)) {
        Some("init") => cli::commands::init(config),
        Some(cmd) => eprintln!("Unknown command: {cmd}"),
        None => eprintln!("Usage: vpsctl <command>"),
    };
}

