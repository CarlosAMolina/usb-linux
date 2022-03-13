use std::env;
use std::process;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let _ = run(config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });

}

fn run(config: Config) -> Result<(), &'static str> {
    let suffix_device_path = "/dev";
    let devices = Devices {
        partition: String::from(&config.partition_device),
        raw: String::from(&config.partition_device[..config.partition_device.len() - 1]),
    };

    let paths = Paths {
        suffix_device: String::from(suffix_device_path),
        raw_device: format!("{suffix_device_path}/{}", devices.raw),
        partition_device: format!("{suffix_device_path}/{}", config.partition_device),
        file_system: String::from("/media/usb"),
    };

    print_summary(&devices, &paths);
    match &config.start_or_end[..] {
        "on" => {
            println!("## Init start USB");
            println!();
            // https://linuxconfig.org/howto-mount-usb-drive-in-linux
            println!(
                "Init mount device {} on {}",
                paths.partition_device, paths.file_system
            );
            run_command(&format!(
                "sudo mount {} {}",
                paths.partition_device, paths.file_system
            ));
            print_system_current_status(&devices.raw, &paths.suffix_device);
        }
        "off" => {
            println!("## Init end USB");
            println!();
            println!("Init umount {}", paths.file_system);
            run_command(&format!("sudo umount {}", paths.file_system));
            print_system_current_status(&devices.raw, &paths.suffix_device);
            println!();
            println!("Init eject {}", paths.raw_device);
            run_command(&format!("sudo eject {}", paths.raw_device));
            println!();
            print_system_current_status(&devices.raw, &paths.suffix_device);
            // https://unix.stackexchange.com/questions/35508/eject-usb-drives-eject-command#83587
            println!("Init power off {}", paths.raw_device);
            run_command(&format!("udisksctl power-off -b {}", paths.raw_device));
            print_system_current_status(&devices.raw, &paths.suffix_device);
        }
        _ => {
            help();
            return Err("invalid command");
        }
    }
    Ok(())
}

struct Config {
    partition_device: String,
    start_or_end: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            help();
            return Err("not enough arguments");
        }
        let partition_device = args[1].clone();
        let start_or_end = args[2].clone();

        Ok(Config {
            partition_device,
            start_or_end,
        })
    }
}

struct Devices {
    partition: String,
    raw: String,
}

struct Paths {
    suffix_device: String,
    raw_device: String,
    partition_device: String,
    file_system: String,
}

fn help() {
    println!(
        "Usage:
    cargo run <string> {{on|off}}
        Start or end an USB device.
Example:
    cargo run sdc1 on"
    );
}

fn print_summary(devices: &Devices, paths: &Paths) {
    // https://serverfault.com/questions/338937/differences-between-dev-sda-and-dev-sda1
    println!("## Summary");
    println!();
    println!("- Raw device: {}", devices.raw);
    println!("- Raw device path: {}", paths.raw_device);
    println!("- Partition device: {}", devices.partition);
    println!("- Partition device path: {}", paths.partition_device);
    println!("- File system path: {}", paths.file_system);
    println!();
    print_system_current_status(&devices.raw, &paths.suffix_device);
}

fn print_system_current_status(raw_device: &str, suffix_device_path: &str) {
    println!("## System current status");
    println!();
    println!("### Devices");
    run_command(&format!("ls {suffix_device_path} | grep {raw_device}"));
    println!("### Mount status");
    run_command(&format!("mount | grep {raw_device}"));
}

fn run_command(c: &str) {
    let output = Command::new("bash")
        .arg("-c")
        .arg(c)
        .output()
        .expect("failed to execute process");
    if output.stderr.len() > 0 {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
