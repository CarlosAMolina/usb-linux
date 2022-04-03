use std::process::Command;

pub type CommandResult = Result<String, String>;

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
    let output_str = String::from_utf8_lossy(&output.stdout); // TODO
    println!("{}", output_str); // TODO
    //let v: Vec<&str> = output_str.split(' ').collect(); // TODO
    //println!("{:?}", v); // TODO
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
            Ok(command_result) => {
                panic!("Error not raised");
            }
            Err(_error) => {}
        };
    }
}
