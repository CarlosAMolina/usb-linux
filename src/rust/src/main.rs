use std::env;
use std::process;

use rust::Config;
use rust::monitor::Config as MonitorConfig;

fn main() {
    if let Err(e) = rust::command_line::clear() {
        eprintln!("Problem clear terminal: {}", e);
        process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    // TODO logic repated in lib.rs
    if args.len() < 3 {
        println!("Init monitor");
        let config = MonitorConfig::new(&args).unwrap_or_else(|e| {
            eprintln!("Problem parsing arguments: {}", e);
            process::exit(1);
        });

        if let Err(e) = rust::monitor::run(config) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        };
    } else {
        println!("Init on-off");
        let config = Config::new(&args).unwrap_or_else(|e| {
            eprintln!("Problem parsing arguments: {}", e);
            process::exit(1);
        });

        if let Err(e) = rust::run(config) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        };
    }
}
