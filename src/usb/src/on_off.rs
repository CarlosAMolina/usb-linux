use crate::command_line;

pub fn run(config: Config) -> command_line::command::CommandResult {
    let devices_and_paths = DevicesAndPaths::new(&config);
    devices_and_paths.print_summary();
    devices_and_paths.print_system_current_status()?;
    match &config.start_or_end[..] {
        "on" => {
            println!("Init start USB");
            println!("==============");
            println!();
            command_line::mount_device(&devices_and_paths.paths.partition_device)?;
            devices_and_paths.print_system_current_status()?;
        }
        "off" => {
            println!("Init end USB");
            println!("============");
            println!();
            command_line::command::run(&format!(
                "udisksctl unmount -b {}",
                devices_and_paths.paths.partition_device
            ))?;
            devices_and_paths.print_system_current_status()?;
            println!();
            command_line::command::run(&format!(
                "udisksctl power-off -b {}",
                devices_and_paths.paths.raw_device
            ))?;
            devices_and_paths.print_system_current_status()?;
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
}

struct Paths {
    raw_device: String,
    partition_device: String,
}

impl Paths {
    pub fn new(config: &Config, devices: &Devices) -> Paths {
        Paths {
            raw_device: devices.raw.to_string(),
            partition_device: config.partition_device.to_string(),
        }
    }
}

struct DevicesAndPaths {
    devices: Devices,
    paths: Paths,
}

impl DevicesAndPaths {
    fn new(config: &Config) -> DevicesAndPaths {
        DevicesAndPaths {
            devices: Devices::new(&config),
            paths: Paths::new(&config, &Devices::new(&config)),
        }
    }

    // https://serverfault.com/questions/338937/differences-between-dev-sda-and-dev-sda1
    fn print_summary(&self) {
        println!("Summary");
        println!("=======");
        println!();
        println!("- Raw device: {}", self.devices.raw);
        println!("- Raw device path: {}", self.paths.raw_device);
        println!("- Partition device: {}", self.devices.partition);
        println!("- Partition device path: {}", self.paths.partition_device);
        println!();
    }

    fn print_system_current_status(&self) -> command_line::command::CommandResult {
        println!("System current status");
        println!("---------------------");
        println!();
        println!("Devices status");
        println!("~~~~~~~~~~~~~~");
        command_line::command::run(&format!("ls /dev/* | grep {}", &self.devices.raw))?;
        println!("Mount status");
        println!("~~~~~~~~~~~~~~");
        command_line::command::run(&format!("mount | grep {}", &self.devices.raw))?;
        Ok("Ok".to_string())
    }
}
