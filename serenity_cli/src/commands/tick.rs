use crate::{App, command};

pub fn tick(_: &str, app: &mut App) -> command::Result<()> {
    app.sheet
        .conditions
        .conditions
        .iter_mut()
        .for_each(|f| f.tick());

    Ok(())
}
