use std::{ops::IndexMut, str::Split};

use serenity_types::meter::Meter;

use crate::{
    App,
    command::{self, CommandError},
    search::{self, filter},
};

pub fn meter(cmd: &str, app: &mut App) -> command::Result<()> {
    let mut args = cmd.split(" ");
    let _first = args.next().expect("Command always has a first value");

    let Some(subcommand) = args.next() else {
        return Err(command::CommandError::ArgumentError);
    };

    match subcommand {
        "add" | "a" => add(args, app)?,
        "remove" | "r" | "d" => remove(args, app)?,
        _ => return Err(CommandError::ArgumentError),
    }

    Ok(())
}
pub fn add(mut args: Split<&str>, app: &mut App) -> command::Result<()> {
    if let Some(name) = args.next() {
        let n: Option<u32> = {
            if let Some(n) = args.next()
                && let Ok(n) = n.parse::<u32>()
            {
                Some(n)
            } else {
                None
            }
        };

        let meter = Meter::new(n.unwrap_or(1), Default::default());

        _ = app.sheet.meters.meters.insert(name.to_string(), meter);

        Ok(())
    } else {
        Err(command::CommandError::ArgumentError)
    }
}

pub fn remove(mut args: Split<&str>, app: &mut App) -> command::Result<()> {
    let Some(term) = args.next() else {
        return Err(CommandError::ArgumentError);
    };

    let data = app
        .sheet
        .meters
        .meters
        .keys()
        .map(|f| f.as_str())
        .collect::<Vec<&str>>();

    let results = search::filter(term, &data, 1);
    if results.is_empty() {
        return Err(CommandError::NotFound);
    }

    let item = data[results[0]].to_string();

    app.sheet.meters.meters.remove(&item);

    Ok(())
}

pub fn spend(cmd: &str, app: &mut App) -> command::Result<()> {
    let mut args = cmd.split(" ");
    let _first = args.next().expect("Command always has a first value");

    if let Some(name) = args.next()
        && let name = name.to_lowercase()
        && let mut filter = app
            .sheet
            .meters
            .meters
            .iter_mut()
            .filter(|f| f.0.to_lowercase().starts_with(&name))
        && let Some(f) = filter.next()
    {
        _ = f.1.spend();
        Ok(())
    } else {
        Err(CommandError::NotFound)
    }
}

pub fn restore(cmd: &str, app: &mut App) -> command::Result<()> {
    let mut args = cmd.split(" ");
    let _first = args.next().expect("Command always has a first value");

    if let Some(name) = args.next()
        && let mut filter = app
            .sheet
            .meters
            .meters
            .iter_mut()
            .filter(|f| f.0.starts_with(&name))
        && let Some(f) = filter.next()
    {
        _ = f.1.restore();
        Ok(())
    } else {
        Err(CommandError::NotFound)
    }
}
