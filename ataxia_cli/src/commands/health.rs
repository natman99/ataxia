use crate::{App, command::CommandError};

pub fn heal(command: &str, state: &mut App) -> crate::command::Result<()> {
    let args: Vec<&str> = command.split(" ").collect();

    if let Some(n) = args.get(1)
        && let Ok(n) = n.parse()
    {
        state.sheet.health.heal(n);
    } else {
        return Err(CommandError::ArgumentError);
    }

    Ok(())
}

pub fn damage(command: &str, state: &mut App) -> crate::command::Result<()> {
    let args: Vec<&str> = command.split(" ").collect();

    if let Some(n) = args.get(1)
        && let Ok(n) = n.parse()
    {
        state.sheet.health.hit(n);
    } else {
        return Err(CommandError::ArgumentError);
    }

    Ok(())
}
