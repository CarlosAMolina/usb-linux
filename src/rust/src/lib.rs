use std::process::Command;

pub fn run(config: Config) -> Result<(), String> {
    let devices = Devices::new(&config);
    let paths = Paths::new(&config, &devices);
    // https://serverfault.com/questions/338937/differences-between-dev-sda-and-dev-sda1
    println!("Summary");
    println!("=======");
    println!();
    println!("- Raw device: {}", devices.raw);
    println!("- Raw device path: {}", paths.raw_device);
    println!("- Partition device: {}", devices.partition);
    println!("- Partition device path: {}", paths.partition_device);
    println!("- File system path: {}", paths.file_system);
    println!();
    if let Err(e) = print_system_current_status(&devices.raw, &paths.suffix_device) {
        return Err(e);
    }
    match &config.start_or_end[..] {
        "on" => {
            println!("Init start USB");
            println!("==============");
            println!();
            // https://linuxconfig.org/howto-mount-usb-drive-in-linux
            if let Err(e) = run_command(&format!(
                "sudo mount {} {}",
                paths.partition_device, paths.file_system
            )) {
                return Err(e);
            }
            if let Err(e) = print_system_current_status(&devices.raw, &paths.suffix_device) {
                return Err(e);
            }
        }
        "off" => {
            println!("Init end USB");
            println!("============");
            println!();
            if let Err(e) = run_command(&format!("sudo umount {}", paths.file_system)) {
                return Err(e);
            };
            if let Err(e) = print_system_current_status(&devices.raw, &paths.suffix_device) {
                return Err(e);
            }
            println!();

            if let Err(e) = run_command(&format!("sudo eject {}", paths.raw_device)) {
                return Err(e);
            };
            println!();
            if let Err(e) = print_system_current_status(&devices.raw, &paths.suffix_device) {
                return Err(e);
            }
            // https://unix.stackexchange.com/questions/35508/eject-usb-drives-eject-command#83587
            if let Err(e) = run_command(&format!("udisksctl power-off -b {}", paths.raw_device)) {
                return Err(e);
            };
            if let Err(e) = print_system_current_status(&devices.raw, &paths.suffix_device) {
                return Err(e);
            }
        }
        _ => {
            help();
            return Err("invalid command".to_string());
        }
    }
    Ok(())
}

pub struct Config {
    partition_device: String,
    start_or_end: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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

impl Devices {
    pub fn new(config: &Config) -> Devices {
        Devices {
            partition: String::from(&config.partition_device),
            raw: String::from(&config.partition_device[..config.partition_device.len() - 1]),
        }
    }
}

struct Paths {
    suffix_device: String,
    raw_device: String,
    partition_device: String,
    file_system: String,
}

impl Paths {
    pub fn new(config: &Config, devices: &Devices) -> Paths {
        let suffix_device_path = "/dev";
        Paths {
            suffix_device: String::from(suffix_device_path),
            raw_device: format!("{suffix_device_path}/{}", devices.raw),
            partition_device: format!("{suffix_device_path}/{}", config.partition_device),
            file_system: String::from("/media/usb"),
        }
    }
}

fn help() {
    eprintln!(
        "Usage:
    cargo run <string> {{on|off}}
        Start or end an USB device.
Example:
    cargo run sdc1 on"
    );
}

fn print_system_current_status(raw_device: &str, suffix_device_path: &str) -> Result<(), String> {
    println!("System current status");
    println!("---------------------");
    println!();
    println!("Devices status");
    println!("~~~~~~~~~~~~~~");
    if let Err(e) = run_command(&format!("ls {suffix_device_path} | grep {raw_device}")) {
        return Err(e);
    }
    println!("Mount status");
    println!("~~~~~~~~~~~~~~");
    if let Err(e) = run_command(&format!("mount | grep {raw_device}")) {
        return Err(e);
    }
    Ok(())
}

fn run_command(c: &str) -> Result<(), String> {
    println!("Init: {}", c);
    let output = Command::new("bash")
        .arg("-c")
        .arg(c)
        .output()
        .expect("failed to execute process");
    if output.stderr.len() > 0 {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
