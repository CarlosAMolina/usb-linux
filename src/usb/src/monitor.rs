use crate::command_line;

pub fn run(config: Config) -> command_line::command::CommandResult {
    let path = String::from(&config.path);
    println!("Path to check: {}", path);
    if must_notify_the_path(&path) {
        println!("Notify path: yes");
        let mounted_path = command_line::mount_device(&format!("/dev/{}", path))?;
        notify(&path, &mounted_path)?;
    } else {
        println!("Notify path: no");
    }
    Ok("Ok".to_string())
}

pub struct Config {
    path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("not enough arguments");
        }
        let path = args[1].clone();
        Ok(Config { path })
    }
}

fn must_notify_the_path(path: &str) -> bool {
    return path.starts_with("sd") & path.chars().last().unwrap().is_digit(10);
}

fn notify(device: &str, mounted_path: &str) -> command_line::command::CommandResult {
    let icon = "/usr/share/icons/Adwaita/48x48/devices/drive-removable-media.png";
    command_line::command::run(&format!(
        "notify-send 'New device' '{}\nMounted at {}' -u normal -i '{}'",
        &device, &mounted_path, &icon
    ))?;
    Ok("Ok".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_notify_the_path_is_true() {
        assert_eq!(true, must_notify_the_path(&"sda1".to_string()));
    }

    #[test]
    fn must_notify_the_path_is_false() {
        assert_eq!(false, must_notify_the_path(&"sda".to_string()));
    }

    #[test]
    fn notify_runs_ok() {
        notify("sda1", "/media/foo/asdfasdfasdfasdfasfasfsaf").unwrap();
    }
}
