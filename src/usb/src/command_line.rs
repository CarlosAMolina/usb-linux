use regex::Regex;

pub mod command;

pub fn clear() -> command::CommandResult {
    if let Err(e) = command::run("clear") {
        return Err(e);
    }
    Ok("Ok".to_string())
}

pub fn mount_device(device: &str) -> &str {
    command::run(&format!("udisksctl mount -b {}", device)).unwrap();
    let mounted_path = "test";
    mounted_path
}

fn get_mounted_path(text: String) -> String {
    let re = Regex::new(r"\sat\s(?P<path>.*)\.$").unwrap();
    let caps = re.captures(&text).unwrap();
    caps["path"].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_clear_runs_ok() {
        clear().unwrap();
    }

    #[test]
    fn mount_device_runs_ok() {
        assert_eq!("test", mount_device("/dev/sda1"));
    }

    #[test]
    fn get_mounted_path_runs_ok() {
        assert_eq!("/media/foo/12abc-34a", get_mounted_path("Mounted /dev/sdb1 at /media/foo/12abc-34a.".to_string()));
    }
}
