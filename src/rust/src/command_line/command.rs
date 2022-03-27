use std::process::Command;

pub type CommandResult = Result<(), String>;

pub fn run(c: &str) -> CommandResult {
    println!("Init: {}", c);
    let output = Command::new("bash")
        .arg("-c")
        .arg(c)
        .output()
        .expect("failed to execute process");
    if output.stderr.len() > 0 {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
