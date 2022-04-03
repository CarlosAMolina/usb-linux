use crate::command_line::command;
//TODO automount) use crate::on_off::Config as OnOffConfig;

pub fn run(config: Config) -> command::CommandResult {
    let path = String::from(&config.path);
    println!("Path to check: {}", path);
    if must_notify_the_path(&path) {
        println!("Notify path: yes");
        notify(&path)?;
        // TODO automount) let config = OnOffConfig::new(&["".to_string(), path, "on".to_string()])?;
        // TODO aouto open) command::run("open /media/usb/ &")?;
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

fn notify(path: &str) -> command::CommandResult {
    let icon = "/usr/share/icons/Adwaita/48x48/devices/drive-removable-media.png";
    // "notify-send 'New device' '{}' -u normal -i '{}'", // TODO add for ubuntu
    command::run(&format!(
        "dunstify 'New device' '{}' -u normal -i '{}'",
        &path, &icon
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
        notify("sda1").unwrap();
    }
}
