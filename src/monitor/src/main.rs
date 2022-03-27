use std::env;
use std::process;

use monitor::monitor::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    if let Err(e) = monitor::monitor::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}

