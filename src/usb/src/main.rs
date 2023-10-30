use log;
use log4rs;
use std::env;
use std::process;

use usb::file;
use usb::monitor::Config as MonitorConfig;
use usb::on_off::Config as OnOffConfig;
use usb::process_status_code;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _handle = log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    if args.len() == 2 {
        if args[1] == "-h" || args[1] == "help" {
            help();
            process::exit(process_status_code::OK_EXIT_CODE);
        } else if args[1] == "-s" || args[1] == "summary" {
            log::debug!("Init summary");
            file::show_file(file::CSV_FILE_PATH_NAME);
            process::exit(process_status_code::OK_EXIT_CODE);
        } else {
            log::debug!("Init monitor");
            let config = MonitorConfig::new(&args).unwrap_or_else(|e| {
                log::error!("Problem parsing arguments: {}", e);
                help();
                process::exit(process_status_code::ERROR_EXIT_CODE);
            });
            if let Err(e) = usb::monitor::run(config) {
                log::error!("Application error: {}", e);
                process::exit(process_status_code::ERROR_EXIT_CODE);
            };
        }
    } else if args.len() == 3 {
        log::debug!("Init on/off the usb");
        let config = OnOffConfig::new(&args).unwrap_or_else(|e| {
            log::error!("Problem parsing arguments: {}", e);
            help();
            process::exit(process_status_code::ERROR_EXIT_CODE);
        });
        if let Err(e) = usb::on_off::run(config) {
            log::error!("Application error: {}", e);
            process::exit(process_status_code::ERROR_EXIT_CODE);
        };
    } else {
        log::error!("Problem parsing arguments");
        help();
        process::exit(process_status_code::ERROR_EXIT_CODE);
    }
}

fn help() {
    println!(
        "Usage
    Option help
        cargo run -- {{-h|help}}
            Shows help.
        Example:
            cargo run -- -h
    Option summary
        cargo run -- {{-s|summary}}
            Shows the devices currently mounted by this project.
        Example:
            cargo run -- -s
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
