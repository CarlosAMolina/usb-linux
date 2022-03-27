pub mod command;

pub fn clear() -> command::CommandResult {
    if let Err(e) = command::run("clear") {
        return Err(e);
    }
    Ok(())
}
