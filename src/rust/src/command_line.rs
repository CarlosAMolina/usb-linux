pub mod command;

pub fn clear() -> Result<(), String> {
    if let Err(e) = command::run("clear") {
        return Err(e);
    }
    Ok(())
}

