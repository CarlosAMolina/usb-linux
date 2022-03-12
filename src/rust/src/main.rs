use std::process::Command;

fn main() {
    let partition_device = "sdc1";
    let raw_device = &partition_device[..partition_device.len()-1];
    let suffix_device_path = "/dev";
    let raw_device_path = format!("{suffix_device_path}/{raw_device}");
    let partition_device_path =  format!("{suffix_device_path}/{partition_device}");
    let file_system_path = "/mnt";

    println!("## Summary");
    println!("Raw device: {}", raw_device);
    println!("Raw device path: {}", raw_device_path);
    println!("Partition device: {}", partition_device);
    println!("Partition device path: {}", partition_device_path);
    println!("File system path: {}", file_system_path);
    println!();


    println!("## System current status");
    println!();
    println!("### Devices status");
    run_command(&format!("ls {raw_device_path}*"));
    println!();
    print_mount_status(&raw_device);

    run_mount(&partition_device_path, &file_system_path, &raw_device);
    //println!("- Mount");
    //println!("{}", String::from_utf8_lossy(&output.stdout));

    //let output = Command::new("bash")
    //        .arg("-c")
    //        .arg(format!("ls /dev | grep {raw_device}"))
    //        .output()
    //        .expect("failed to execute process");
    //println!("## Devices");
    //println!("{}", String::from_utf8_lossy(&output.stdout));

    //let output = Command::new("bash")
    //        .arg("-c")
    //        .arg(format!("mount | grep {raw_device}"))
    //        .output()
    //        .expect("failed to execute process");
    //println!("## Mount");
    //println!("{}", String::from_utf8_lossy(&output.stdout));

}


fn print_mount_status(raw_device: &str) {
    println!("### Mount status");
    run_command(&format!("mount | grep {raw_device}"));
}

fn run_mount(partition_device_path: &str, file_system_path: &str, raw_device: &str){
    println!("Init mount device {} on {}", partition_device_path, file_system_path);
    run_command(&format!("sudo mount {partition_device_path} {file_system_path}"));
    print_mount_status(&raw_device);
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


