use crate::command_line;
use std::path::Path;

pub fn get_mount_status(device: &String) -> command_line::command::CommandResult {
    command_line::command::run(&format!("mount | grep {}", device))
}

pub fn show_devices() {
    let mut devices = command_line::command::run(&format!("ls /dev/sd*")).unwrap();
    devices = devices.replace("\n", " ");
    println!("{}", devices);
}

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
