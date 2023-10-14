use std::path::Path;

use crate::command_line;

pub fn run(config: Config) -> command_line::command::CommandResult {
    let devices = Devices::new(&config);
    devices.show_summary();
    devices.show_system_current_status()?;
    match &config.start_or_end[..] {
        "on" => {
            log::info!("Init on USB");
            mount(&devices)?;
        }
        "off" => {
            log::info!("Init off USB");
            unmount(&devices)?;
            power_off(&devices)?;
        }
        _ => {
            return Err("invalid command".to_string());
        }
    }
    Ok("Ok".to_string())
}

fn mount(devices: &Devices) -> command_line::command::CommandResult {
    let device_path = &devices.partition;
    log::debug!("Init mount {}", device_path);
    // TODO duplicated line, extract to common method
    let mount_status = command_line::command::run(&format!("mount | grep {}", devices.raw))?;
    if mount_status.is_empty() {
        if Path::new(device_path).exists() {
            command_line::mount_device(&devices.partition)?;
        } else {
            log::debug!("No device to manage");
        }
    } else {
        log::debug!("Device already mounted");
        // TODO notify device and mounted path
    }
    Ok("Ok".to_string())
}

fn unmount(devices: &Devices) -> command_line::command::CommandResult {
    let device_path = &devices.partition;
    log::debug!("Init unmount {}", device_path);
    if Path::new(device_path).exists() {
        command_line::command::run(&format!("udisksctl unmount -b {}", device_path))?;
        devices.show_system_current_status()?;
    } else {
        log::debug!("No mounted device to manage");
    }
    Ok("Ok".to_string())
}

fn power_off(devices: &Devices) -> command_line::command::CommandResult {
    let device_path = &devices.raw;
    log::debug!("Init power off {}", device_path);
    if Path::new(device_path).exists() {
        command_line::command::run(&format!("udisksctl power-off -b {}", device_path))?;
        devices.show_system_current_status()?;
    } else {
        log::debug!("No connected device to manage");
    }
    Ok("Ok".to_string())
}

pub struct Config {
    partition_device: String,
    start_or_end: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
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

    // https://serverfault.com/questions/338937/differences-between-dev-sda-and-dev-sda1
    fn show_summary(&self) {
        let summary = format!(
            "Device to manage:
- Device's raw path: {}
- Device's partition path: {}",
            self.raw, self.partition
        );
        log::debug!("{}", summary);
    }

    fn show_system_current_status(&self) -> command_line::command::CommandResult {
        log::debug!("Init show system current status");
        let system_status = self.get_system_current_status()?;
        log::debug!("{}", system_status);
        Ok("Ok".to_string())
    }

    fn get_system_current_status(&self) -> command_line::command::CommandResult {
        let devices_status =
            command_line::command::run(&format!("ls /dev/* | grep {}", &self.raw))?;
        let mount_status = command_line::command::run(&format!("mount | grep {}", &self.raw))?;
        let result = format!(
            "System current status:
- Connected devices:
{devices_status} 
- Mounted devices:
{mount_status}"
        );
        Ok(result.to_string())
    }
}
