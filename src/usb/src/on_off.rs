use crate::command_line;
use crate::file;
use std::path::Path;

pub fn run(config: Config) -> command_line::command::CommandResult {
    let devices = Devices::new(&config);
    log::debug!("User input device: {}", config.partition_device);
    devices.show_device_summary();
    system::show_system_current_status(&devices.raw)?;
    match &config.start_or_end[..] {
        "on" => {
            log::info!("Init on USB");
            mount(&devices)?;
            system::show_system_current_status(&devices.raw)?;
            log::info!("Completed on USB");
        }
        "off" => {
            log::info!("Init off USB");
            if is_partition(&devices.partition) {
                unmount(&devices)?;
                system::show_system_current_status(&devices.raw)?;
                file::delete_mount_info_in_file(file::CSV_FILE_PATH_NAME, &devices.partition);
            } else {
                log::debug!(
                    "The provided device is not a partition: {}. Omitting unmount",
                    devices.partition
                );
            }
            if is_device_raw(&devices.raw) {
                power_off(&devices)?;
            } else {
                log::debug!("Invalid raw device: {}. Omitting power off", devices.raw);
                return Err("invalid device".to_string());
            }
            system::show_system_current_status(&devices.raw)?;
            log::info!("Completed off USB");
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
    log::debug!("Init mount {}", devices.partition);
    let mount_status = system::get_mount_status(&devices.raw)?;
    if mount_status.is_empty() {
        let device_partition = &devices.partition;
        if Path::new(device_partition).exists() {
            let mounted_path = command_line::mount_device(device_partition)?;
            file::save_mount_info_to_file(file::CSV_FILE_PATH_NAME, device_partition, &mounted_path)
        } else {
            log::info!("The device does not exist: {}", device_partition);
        }
    } else {
        log::info!("{}", "Device already mounted");
        let partition_mount_status = system::get_mount_status(&devices.partition)?;
        let partition_mount_status_to_show =
            get_partition_mount_status_to_show(&partition_mount_status);
        log::info!("{}", partition_mount_status_to_show);
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

fn unmount(devices: &Devices) -> command_line::command::CommandResult {
    let device_path = &devices.partition;
    log::debug!("Init unmount {}", device_path);
    if !system::get_mount_status(device_path).unwrap().is_empty() && Path::new(device_path).exists()
    {
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
}
// TODO move to system.rs
mod system {

    use crate::command_line;
    use std::path::Path;

    pub fn show_system_current_status(
        device_raw_path_name_str: &String,
    ) -> command_line::command::CommandResult {
        log::debug!("Init show system current status");
        let system_status = get_system_current_status(device_raw_path_name_str)?;
        log::debug!("{}", system_status);
        Ok("Ok".to_string())
    }

    fn get_system_current_status(
        device_raw_path_name_str: &String,
    ) -> command_line::command::CommandResult {
        let devices_status = get_devices_system_current_status(device_raw_path_name_str)?;
        let mount_status = get_mount_status(device_raw_path_name_str)?;
        let result = format!(
            "System current status:
- Connected devices:
{devices_status} 
- Mounted devices:
{mount_status}"
        );
        Ok(result.to_string())
    }

    fn get_devices_system_current_status(
        device_raw_path_name_str: &String,
    ) -> command_line::command::CommandResult {
        let device_raw_path_name = Path::new(device_raw_path_name_str);
        let devices_path_name = device_raw_path_name.parent().unwrap().to_str().unwrap();
        let raw_device_name = device_raw_path_name.file_name().unwrap().to_str().unwrap();
        let device_names_str = command_line::command::run(&format!(
            "ls {} | grep {}",
            devices_path_name, raw_device_name
        ))?;
        let device_names_all: Vec<&str> = device_names_str.split("\n").collect();
        let device_names: Vec<_> = device_names_all
            .iter()
            .filter(|name| !name.is_empty())
            .collect();
        let device_path_names: Vec<_> = device_names
            .iter()
            .map(|name| format!("{}/{}", devices_path_name, name))
            .collect();
        let result = device_path_names.join("\n");
        Ok(result)
    }

    pub fn get_mount_status(device: &String) -> command_line::command::CommandResult {
        command_line::command::run(&format!("mount | grep {}", device))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // TODO new test: non existent raw path

        #[test]
        fn get_devices_system_current_status_runs_ok_if_no_permissions_on_a_folder() {
            /*
             * To run this test:
             * ```bash
             * mkdir -p /tmp/usb-tests/dev/sda
             * mkdir /tmp/usb-tests/dev/sda1
             * mkdir /tmp/usb-tests/dev/sda2
             * mkdir /tmp/usb-tests/dev/folder_no_permissions
             * chmod 700 /tmp/usb-tests/dev/folder_no_permissions
             * sudo chown root:root /tmp/usb-tests/dev/folder_no_permissions
             * ```
             */
            let device_raw_path_name = "/tmp/usb-tests/dev/sda".to_string();
            assert_eq!(
                "/tmp/usb-tests/dev/sda\n/tmp/usb-tests/dev/sda1\n/tmp/usb-tests/dev/sda2",
                get_devices_system_current_status(&device_raw_path_name).unwrap()
            );
        }
    }
}

fn get_partition_mount_status_to_show(partition_mount_status: &String) -> String {
    let string_parts: Vec<&str> = partition_mount_status.split_whitespace().collect();
    let result = format!("{} mounted on {}", string_parts[0], string_parts[2]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_partition_mount_status_to_show_gives_expected_result() {
        let partition_mount_status = "/dev/sda4 on / type ext4 (rw,relatime)".to_string();
        assert_eq!(
            "/dev/sda4 mounted on /",
            get_partition_mount_status_to_show(&partition_mount_status)
        );
    }
}
