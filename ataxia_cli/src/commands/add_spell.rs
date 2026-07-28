use std::fs;

use crate::{
    App,
    command::{self, CommandError},
    search,
};

pub fn add(command: &str, app: &mut App) -> command::Result<()> {
    let mut args: Vec<&str> = command.split(" ").collect();
    if args.len() < 2 {
        return Err(CommandError::ArgumentError);
    }
    args.remove(0);

    let term = args.join(" ");

    let results = search::search_spells(term.as_str(), &app.data.spells);

    fs::write("test.txt", format!("{:#?}", results)).unwrap();

    if let Some(ref result) = results.get(0) {
        let spell = **result;
        let spell = spell.clone();

        app.sheet.spells.spells.insert(spell.name.to_owned(), spell);
    } else {
        return Err(CommandError::NotFound);
    }

    Ok(())
}

pub fn remove(command: &str, app: &mut App) -> command::Result<()> {
    let mut args: Vec<_> = command.split(" ").collect();

    if args.len() < 2 {
        return Err(CommandError::ArgumentError);
    }

    args.remove(0);

    let term = args.join(" ");

    let target = {
        let data: Vec<_> = app
            .sheet
            .spells
            .spells
            .iter()
            .map(|f| f.0.as_str())
            .collect();

        let results = search::filter(&term, &data, 1);
        if results.len() != 1 {
            return Err(CommandError::ArgumentError);
        } else {
            data[results[0]].to_string()
        }
    };

    app.sheet.spells.spells.remove(&target);

    Ok(())
}
