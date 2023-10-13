use crate::command_line;

pub fn run(config: Config) -> command_line::command::CommandResult {
    let devices = Devices::new(&config);
    devices.print_summary();
    devices.print_system_current_status()?;
    match &config.start_or_end[..] {
        "on" => {
            log::info!("Init start USB");
            command_line::mount_device(&devices.partition)?;
            devices.print_system_current_status()?;
        }
        "off" => {
            log::info!("Init end USB");
            command_line::command::run(&format!("udisksctl unmount -b {}", devices.partition))?;
            devices.print_system_current_status()?;
            command_line::command::run(&format!("udisksctl power-off -b {}", devices.raw))?;
            devices.print_system_current_status()?;
        }
        _ => {
            return Err("invalid command".to_string());
        }
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
    fn print_summary(&self) {
        let summary = format!(
            "Device to manage:
- Device's raw path: {}
- Device's partition path: {}",
            self.raw, self.partition
        );
        log::debug!("{}", summary);
    }

    fn print_system_current_status(&self) -> command_line::command::CommandResult {
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
