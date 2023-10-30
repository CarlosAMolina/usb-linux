use crate::command_line;

pub fn show_devices() {
    let mut devices = command_line::command::run(&format!("ls /dev/sd*")).unwrap();
    devices = devices.replace("\n", " ");
    println!("{}", devices);
}
