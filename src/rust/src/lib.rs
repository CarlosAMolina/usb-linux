pub mod command_line;

use crate::command_line::command;

pub fn run(config: Config) -> command::CommandResult {
    let devices = Devices::new(&config);
    let paths = Paths::new(&config, &devices);
    let devices_and_paths = DevicesAndPaths {
        devices: &devices,
        paths: &paths,
    };
    devices_and_paths.print_summary();
    devices_and_paths.print_system_current_status()?;
    match &config.start_or_end[..] {
        "on" => {
            println!("Init start USB");
            println!("==============");
            println!();
            // https://linuxconfig.org/howto-mount-usb-drive-in-linux
            command::run(&format!(
                "sudo mount {} {}",
                paths.partition_device, paths.file_system
            ))?;
            devices_and_paths.print_system_current_status()?;
        }
        "off" => {
            println!("Init end USB");
            println!("============");
            println!();
            command::run(&format!("sudo umount {}", paths.file_system))?;
            devices_and_paths.print_system_current_status()?;
            println!();

            command::run(&format!("sudo eject {}", paths.raw_device))?;
            println!();
            devices_and_paths.print_system_current_status()?;
            // https://unix.stackexchange.com/questions/35508/eject-usb-drives-eject-command#83587
            command::run(&format!("udisksctl power-off -b {}", paths.raw_device))?;
            devices_and_paths.print_system_current_status()?;
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

struct DevicesAndPaths<'a> {
    devices: &'a Devices,
    paths: &'a Paths,
}

impl DevicesAndPaths<'_> {
    // https://serverfault.com/questions/338937/differences-between-dev-sda-and-dev-sda1
    fn print_summary(&self) {
        println!("Summary");
        println!("=======");
        println!();
        println!("- Raw device: {}", self.devices.raw);
        println!("- Raw device path: {}", self.paths.raw_device);
        println!("- Partition device: {}", self.devices.partition);
        println!("- Partition device path: {}", self.paths.partition_device);
        println!("- File system path: {}", self.paths.file_system);
        println!();
    }

    fn print_system_current_status(&self) -> command::CommandResult {
        println!("System current status");
        println!("---------------------");
        println!();
        println!("Devices status");
        println!("~~~~~~~~~~~~~~");
        command::run(&format!(
            "ls {} | grep {}",
            &self.paths.suffix_device,
            &self.devices.raw
        ))?;
        println!("Mount status");
        println!("~~~~~~~~~~~~~~");
        command::run(&format!("mount | grep {}", &self.devices.raw))?;
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_command_runs_ok() {
        command::run(&format!("echo hi")).unwrap();
    }

    #[test]
    fn command_line_command_raises_error() {
        let _result = match command::run(&format!("asdf")) {
            Ok(()) => {
                panic!("Error not raised");
            }
            Err(_error) => {}
        };
    }

    #[test]
    fn command_line_clear_runs_ok() {
        command_line::clear().unwrap();
    }
}
