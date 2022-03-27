use crate::command_line::command;

pub fn run(config: Config) -> command::CommandResult {
    let path = String::from(&config.path);
    if must_notify_the_path(&path) {
        notify(&path)?;
    }
    Ok(())
}

pub struct Config {
    path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            help();
            return Err("not enough arguments");
        }
        let path = args[1].clone();

        Ok(Config { path })
    }
}

fn help() {
    eprintln!(
        "Usage:
    cargo run <string>
        Notify a device's path if required.
Example:
    cargo run /dev/sdc1"
    );
}

fn must_notify_the_path(path: &str) -> bool {
    return path.starts_with("/dev/sd") & path.chars().last().unwrap().is_digit(10);
}

pub fn notify(path: &str) -> command::CommandResult {
    let icon = "/usr/share/icons/Adwaita/48x48/devices/drive-removable-media.png";
    command::run(&format!(
        "dunstify 'New device' '{}' -u normal -i '{}'",
        &path, &icon
    ))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_notify_the_path_is_true() {
        assert_eq!(true, must_notify_the_path(&"/dev/sda1".to_string()));
    }

    #[test]
    fn must_notify_the_path_is_false() {
        assert_eq!(false, must_notify_the_path(&"/dev/sda".to_string()));
    }

    #[test]
    fn notify_runs_ok() {
        notify("sda1").unwrap();
    }
}
