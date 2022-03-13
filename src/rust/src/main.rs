use std::env;
use std::process;

use rust::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let _ = rust::run(config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });
}
