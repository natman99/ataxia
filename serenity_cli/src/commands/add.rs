use std::fs;

use edit_distance::edit_distance;
use serenity_types::{Character, database::spell::Spell};

use crate::{
    App,
    command::{self, CommandError},
    commands, search,
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
