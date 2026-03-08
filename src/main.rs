mod cli;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|val| String::as_str(val)) {
        Some("init") => cli::commands::init(),
        Some(cmd) => eprintln!("Unknown command: {cmd}"),
        None => eprintln!("Usage: vpsctl <command>"),
    };
}

