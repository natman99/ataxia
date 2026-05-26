use crate::{
    App, FocusedContent,
    command::{self, CommandError},
};

pub fn search(command: &str, app: &mut App) -> command::Result<()> {
    let mut args: Vec<&str> = command.split(" ").collect();
    if args.len() < 2 {
        return Err(CommandError::ArgumentError);
    }
    args.remove(0);

    let term = args.join(" ");

    let results = crate::search::search(&term, &app.data);
    let results = results.join("\n\n");

    app.focused_content = FocusedContent::new(results);

    Ok(())
}
