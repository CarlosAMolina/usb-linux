use env_logger::{Builder, Env, Target};
use std::env;
use std::process;

use usb::monitor::Config as MonitorConfig;
use usb::on_off::Config as OnOffConfig;
use usb::process_status_code;

fn main() {
    let args: Vec<String> = env::args().collect();

    Builder::from_env(Env::default().default_filter_or("debug"))
        .target(Target::Stdout)
        .init();

    if args.len() == 2 {
        if args[1] == "-h" || args[1] == "help" {
            help();
            process::exit(process_status_code::OK_EXIT_CODE);
        } else {
            println!("Init monitor");
            let config = MonitorConfig::new(&args).unwrap_or_else(|e| {
                eprintln!("Problem parsing arguments: {}", e);
                help();
                process::exit(process_status_code::ERROR_EXIT_CODE);
            });
            if let Err(e) = usb::monitor::run(config) {
                eprintln!("Application error: {}", e);
                process::exit(process_status_code::ERROR_EXIT_CODE);
            };
        }
    } else if args.len() == 3 {
        if let Err(e) = usb::command_line::clear() {
            eprintln!("Problem clear terminal: {}", e);
            process::exit(process_status_code::ERROR_EXIT_CODE);
        }
        println!("Init on-off");
        let config = OnOffConfig::new(&args).unwrap_or_else(|e| {
            eprintln!("Problem parsing arguments: {}", e);
            help();
            process::exit(process_status_code::ERROR_EXIT_CODE);
        });
        if let Err(e) = usb::on_off::run(config) {
            eprintln!("Application error: {}", e);
            process::exit(process_status_code::ERROR_EXIT_CODE);
        };
    } else {
        eprintln!("Problem parsing arguments");
        help();
        process::exit(process_status_code::ERROR_EXIT_CODE);
    }
}

fn help() {
    eprintln!(
        "Usage
    Option help
        cargo run -- {{-h|help}}
            Shows help
        Example:
            cargo run -- -h
    Option monitor
        cargo run <string>
            Notify a device's path and mount it if required.
        Example:
            cargo run /dev/sdc1
    Option on_off
        cargo run <string> {{on|off}}
            Start or end an USB device.
        Example:
            cargo run /dev/sdc1 on"
    );
}
