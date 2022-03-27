use std::env;
use std::process;

use usb::monitor::Config as MonitorConfig;
use usb::on_off::Config as OnOffConfig;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Init monitor");
        let config = MonitorConfig::new(&args).unwrap_or_else(|e| {
            eprintln!("Problem parsing arguments: {}", e);
            process::exit(1);
        });
        if let Err(e) = usb::monitor::run(config) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        };
    } else {
        if let Err(e) = usb::command_line::clear() {
            eprintln!("Problem clear terminal: {}", e);
            process::exit(1);
        }
        println!("Init on-off");
        let config = OnOffConfig::new(&args).unwrap_or_else(|e| {
            eprintln!("Problem parsing arguments: {}", e);
            process::exit(1);
        });
        if let Err(e) = usb::on_off::run(config) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        };
    }
}
