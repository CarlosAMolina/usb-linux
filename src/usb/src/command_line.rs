pub mod command;

pub fn clear() -> command::CommandResult {
    if let Err(e) = command::run("clear") {
        return Err(e);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_clear_runs_ok() {
        clear().unwrap();
    }
}

