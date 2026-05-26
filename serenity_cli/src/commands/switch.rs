use crate::{App, widgets::State};

pub fn spell(_command: &str, state: &mut App) -> crate::command::Result<()> {
    state.state = State::Spells;

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
