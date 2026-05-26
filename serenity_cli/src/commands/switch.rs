use crate::{App, widgets::State};

pub fn spell(command: &str, state: &mut App) -> crate::command::Result<()> {
    let split = command.split(" ").collect::<Vec<&str>>();
    let num;
    if let Some(n) = split.get(1) {
        if let Ok(n) = n.parse::<usize>() {
            num = Some(n);
        } else {
            return Err(crate::command::CommandError::ArgumentError);
        }
    } else {
        num = None;
    }

    state.state = State::Spells(num);

    Ok(())
}

pub fn inventory(_command: &str, state: &mut App) -> crate::command::Result<()> {
    state.state = State::Inventory;

    Ok(())
}

pub fn g_main(_command: &str, state: &mut App) -> crate::command::Result<()> {
    state.state = State::Main;

    Ok(())
}
