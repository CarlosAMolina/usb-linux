use std::fs;

fn main() {
    for path in fs::read_dir("/dev").unwrap() {
        let path_str = path.unwrap().path().display().to_string();
        if path_str.starts_with("/dev/sd") & path_str.chars().last().unwrap().is_digit(10) {
            println!("Path: {}", path_str);
        }
    }
}
