use enumflags2::BitFlags;

use crate::{
    App, FocusedContent,
    command::{self, CommandError},
    search::Filters,
};

pub fn search(command: &str, app: &mut App) -> command::Result<()> {
    let mut args: Vec<&str> = command.split(" ").collect();
    if args.len() < 2 {
        return Err(CommandError::ArgumentError);
    }
    args.remove(0);

    let mut flags: BitFlags<Filters> = BitFlags::default();

    let filters = args.iter().position(|f| f.starts_with("-"));

    if let Some(f) = filters {
        let filter = args.remove(f);
        flags = BitFlags::empty();
        if filter.contains("m") {
            flags.insert(Filters::MagicItems);
        }
        if filter.contains("s") {
            flags.insert(Filters::Skills);
            flags.insert(Filters::Spells);
        }

        if filter.contains("t") {
            flags.insert(Filters::Traits);
        }

        if filter.contains("r") {
            flags.insert(Filters::Rules);
        }

        if filter.contains("a") {
            flags = BitFlags::all();
        }
    }

    let term = args.join(" ");

    let results = crate::search::search(&term, &app.data, flags);
    let results = results.join("\n\n");

    app.focused_content = FocusedContent::new(results);

    Ok(())
}
