use regex::Regex;

pub mod command;

pub fn mount_device(device: &str) -> command::CommandResult {
    let mounted_path = "/foo/test".to_string(); // TODO
    // TODO let command_result = command::run(&format!("udisksctl mount -b {}", device))?;
    // TODO let mounted_path = get_mounted_path(command_result);
    log::debug!("Mounted at: {}", mounted_path);
    Ok(mounted_path)
}

fn get_mounted_path(text: String) -> String {
    let re = Regex::new(r"\sat\s(?P<path>.*)\n$").unwrap();
    let caps = re.captures(&text).unwrap();
    caps["path"].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn mount_device_runs_ok() {
        mount_device("/dev/sdb1").unwrap();
    }

    #[test]
    fn get_mounted_path_runs_ok() {
        assert_eq!(
            "/media/foo/12abc-34a",
            get_mounted_path("Mounted /dev/sdb1 at /media/foo/12abc-34a\n".to_string())
        );
    }
}
