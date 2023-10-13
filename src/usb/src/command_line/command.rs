use std::process::Command;

pub type CommandResult = Result<String, String>;

pub fn run(c: &str) -> CommandResult {
    log::debug!("Init: {}", c);
    let output = Command::new("bash")
        .arg("-c")
        .arg(c)
        .output()
        .expect("failed to execute process");
    if output.stderr.len() > 0 {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    let output_str = String::from_utf8_lossy(&output.stdout);
    Ok(output_str.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_runs_ok() {
        assert_eq!("hi\n", run("echo hi").unwrap());
    }

    #[test]
    fn run_raises_error() {
        let _result = match run(&format!("asdf")) {
            Ok(_command_result) => {
                panic!("Error not raised");
            }
            Err(_error) => {}
        };
    }
}
