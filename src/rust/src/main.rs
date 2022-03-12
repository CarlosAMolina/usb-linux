use std::process::Command;

fn main() {
    let partition_device = "sdc1";
    let raw_device = &partition_device[..partition_device.len()-1];
    let suffix_device_path = "/dev";
    let raw_device_path = format!("{suffix_device_path}/{raw_device}");
    let partition_device_path =  format!("{suffix_device_path}/{partition_device}");
    let file_system_path = "/media/usb";
    let start_usb = !true;

    // https://serverfault.com/questions/338937/differences-between-dev-sda-and-dev-sda1
    println!("## Summary");
    println!();
    println!("- Raw device: {}", raw_device);
    println!("- Raw device path: {}", raw_device_path);
    println!("- Partition device: {}", partition_device);
    println!("- Partition device path: {}", partition_device_path);
    println!("- File system path: {}", file_system_path);
    println!();


    println!("## System current status");
    println!();
    print_devices_status(&raw_device, &suffix_device_path);
    println!();
    print_mount_status(&raw_device);

    if start_usb {
        println!("## Init start USB");
        println!();
        run_mount(&partition_device_path, &file_system_path);
        print_mount_status(&raw_device);
    } else {
        println!("## Init end USB");
        println!();
        println!("Init umount {}", file_system_path);
        run_command(&format!("sudo umount {file_system_path}"));
        print_mount_status(&raw_device);
        println!();
        println!("Init eject {}", raw_device_path);
        run_command(&format!("sudo eject {raw_device_path}"));
        println!();
        print_devices_status(&raw_device, &suffix_device_path);
        // https://unix.stackexchange.com/questions/35508/eject-usb-drives-eject-command#83587
        println!("### Init power off {}", raw_device_path);
        run_command(&format!("udisksctl power-off -b {raw_device_path}"));
        print_devices_status(&raw_device, &suffix_device_path);
    }

}

fn print_devices_status(raw_device: &str, suffix_device_path: &str) {
    println!("### Devices");
    run_command(&format!("ls {suffix_device_path} | grep {raw_device}"));
}

fn print_mount_status(raw_device: &str) {
    println!("### Mount status");
    run_command(&format!("mount | grep {raw_device}"));
}

// https://linuxconfig.org/howto-mount-usb-drive-in-linux
fn run_mount(partition_device_path: &str, file_system_path: &str){
    println!("Init mount device {} on {}", partition_device_path, file_system_path);
    run_command(&format!("sudo mount {partition_device_path} {file_system_path}"));
}

fn run_command(c: &str) {
    let output = Command::new("bash")
            .arg("-c")
            .arg(c)
            .output()
            .expect("failed to execute process");
    if output.stderr.len() > 0 {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }
    println!("{}", String::from_utf8_lossy(&output.stdout));
}


