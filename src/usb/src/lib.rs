pub mod command_line;
pub mod on_off;
pub mod monitor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_clear_runs_ok() {
        command_line::clear().unwrap();
    }
}

