use crate::{App, FocusedContent, command};

pub fn help(command: &str, state: &mut App) -> command::Result<()> {
    let a = crate::COMMAND_HANDLER;
    let b = a.lock();
    let c = b.iter().map(|f| f.key()).collect::<Vec<&str>>().join("\n");
    state.focused_content = FocusedContent::new(c);

    Ok(())
}
