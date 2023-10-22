use std::path::Path;

use crate::command_line;
use crate::file;

pub fn run(config: Config) -> command_line::command::CommandResult {
    let devices = Devices::new(&config);
    log::debug!("User input device: {}", config.partition_device);
    devices.show_device_summary();
    devices.show_system_current_status()?;
    match &config.start_or_end[..] {
        "on" => {
            log::info!("Init on USB");
            mount(&devices)?;
            devices.show_system_current_status()?;
            log::debug!("Completed on USB");
        }
        "off" => {
            log::info!("Init off USB");
            file::delete_mount_info_in_file(&devices.partition);
            // TODO
            //if is_partition(&devices.partition) {
            //    unmount(&devices)?;
            //    devices.show_system_current_status()?;
            //    delete_mount_info_in_file(&devices.partition);
            //} else {
            //    log::debug!(
            //        "The provided device is not a partition: {}. Omitting unmount",
            //        devices.partition
            //    );
            //}
            //if is_device_raw(&devices.raw) {
            //    power_off(&devices)?;
            //} else {
            //    log::debug!("Invalid raw device: {}. Omitting power off", devices.raw);
            //    return Err("invalid device".to_string());
            //}
            //devices.show_system_current_status()?;
            //log::debug!("Completed off USB");
        }
        _ => {
            return Err("invalid command".to_string());
        }
    }
    Ok("Ok".to_string())
}

fn mount(devices: &Devices) -> command_line::command::CommandResult {
    if !is_partition(&devices.partition) {
        log::warn!(
            "The provided device must end in a number: {}. Omitting mount",
            devices.partition
        );
        return Err("invalid partition device".to_string());
    }
    let device_path = &devices.partition;
    log::debug!("Init mount {}", device_path);
    let mount_status = get_mount_status(devices)?;
    if mount_status.is_empty() {
        if Path::new(&device_path).exists() {
            let mounted_path = command_line::mount_device(&devices.partition)?;
            file::save_mount_info_to_file(&device_path, &mounted_path)
        } else {
            log::debug!("No device to manage");
        }
    } else {
        log::debug!("Device already mounted");
        // TODO notify device and mounted path
    }
    Ok("Ok".to_string())
}

fn is_partition(string: &String) -> bool {
    let partition_last_character = string.chars().last().unwrap();
    partition_last_character.is_digit(10)
}

fn is_device_raw(string: &String) -> bool {
    if is_partition(string) {
        return true;
    } else {
        let path = Path::new(string);
        let raw_name = path.file_name().unwrap();
        let result = raw_name.len() == 3;
        result
    }
}

fn get_mount_status(devices: &Devices) -> command_line::command::CommandResult {
    command_line::command::run(&format!("mount | grep {}", devices.raw))
}

fn unmount(devices: &Devices) -> command_line::command::CommandResult {
    let device_path = &devices.partition;
    log::debug!("Init unmount {}", device_path);
    if Path::new(device_path).exists() {
        command_line::command::run(&format!("udisksctl unmount -b {}", device_path))?;
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
    fn show_device_summary(&self) {
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
        let mount_status = get_mount_status(&self)?;
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
