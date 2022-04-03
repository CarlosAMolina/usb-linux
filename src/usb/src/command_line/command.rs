use std::process::Command;

use regex::Regex;

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
    let output_str = String::from_utf8_lossy(&output.stdout); // TODO
    println!("{}", output_str); // TODO
    //let v: Vec<&str> = output_str.split(' ').collect(); // TODO
    //println!("{:?}", v); // TODO
    Ok(())
}

fn get_mounted_path(text: String) -> String {
    let re = Regex::new(r"\sat\s(?P<path>.*)\.$").unwrap();
    let caps = re.captures(&text).unwrap();
    caps["path"].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_command_runs_ok() {
        run(&format!("udisksctl mount -b /dev/sda1")).unwrap();
    }

    #[test]
    fn get_mounted_path_runs_ok() {
        assert_eq!("/media/foo/12abc-34a", get_mounted_path("Mounted /dev/sdb1 at /media/foo/12abc-34a.".to_string()));
    }

    #[test]
    fn command_line_command_raises_error() {
        let _result = match run(&format!("asdf")) {
            Ok(()) => {
                panic!("Error not raised");
            }
            Err(_error) => {}
        };
    }
}
