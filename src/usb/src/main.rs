use std::env;
use std::process;

use usb::monitor::Config as MonitorConfig;
use usb::on_off::Config as OnOffConfig;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        if args[1] == "-h" {
            help();
            process::exit(0);
        } else {
            println!("Init monitor");
            let config = MonitorConfig::new(&args).unwrap_or_else(|e| {
                eprintln!("Problem parsing arguments: {}", e);
                help();
                process::exit(1);
            });
            if let Err(e) = usb::monitor::run(config) {
                eprintln!("Application error: {}", e);
                process::exit(1);
            };
        }
    } else if args.len() == 3 {
        if let Err(e) = usb::command_line::clear() {
            eprintln!("Problem clear terminal: {}", e);
            process::exit(1);
        }
        println!("Init on-off");
        let config = OnOffConfig::new(&args).unwrap_or_else(|e| {
            eprintln!("Problem parsing arguments: {}", e);
            help();
            process::exit(1);
        });
        if let Err(e) = usb::on_off::run(config) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        };
    } else {
        eprintln!("Problem parsing arguments");
        help();
        process::exit(1);
    }
}

fn help() {
    eprintln!(
        "Usage
    Option help
        cargo run -- -h
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
