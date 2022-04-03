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
            command_line::mount_device(
                &devices_and_paths.paths.partition_device
            )?;
            devices_and_paths.print_system_current_status()?;
        }
        "off" => {
            println!("Init end USB");
            println!("============");
            println!();
            command_line::unmount_device(
                &devices_and_paths.paths.partition_device
            )?;
            devices_and_paths.print_system_current_status()?;
            println!();

            // udisksctl unmount -b /dev/sda1
            command_line::command::run(&format!(
                "sudo eject {}",
                devices_and_paths.paths.raw_device
            ))?;
            println!();
            devices_and_paths.print_system_current_status()?;
            // https://unix.stackexchange.com/questions/35508/eject-usb-drives-eject-command#83587
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
    suffix_device: String,
    raw_device: String,
    partition_device: String,
    file_system: String,
}

impl Paths {
    pub fn new(config: &Config, devices: &Devices) -> Paths {
        let suffix_device_path = "/dev";
        Paths {
            suffix_device: String::from(suffix_device_path),
            raw_device: format!("{suffix_device_path}/{}", devices.raw),
            partition_device: format!("{suffix_device_path}/{}", config.partition_device),
            file_system: String::from("/media/usb"),
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
        println!("- File system path: {}", self.paths.file_system);
        println!();
    }

    fn print_system_current_status(&self) -> command_line::command::CommandResult {
        println!("System current status");
        println!("---------------------");
        println!();
        println!("Devices status");
        println!("~~~~~~~~~~~~~~");
        command_line::command::run(&format!(
            "ls {} | grep {}",
            &self.paths.suffix_device, &self.devices.raw
        ))?;
        println!("Mount status");
        println!("~~~~~~~~~~~~~~");
        command_line::command::run(&format!("mount | grep {}", &self.devices.raw))?;
        Ok("Ok".to_string())
    }
}
