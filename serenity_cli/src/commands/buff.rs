use std::str::Split;

use serenity_types::condition::Condition;

use crate::{
    App,
    command::{self, CommandError},
    search,
};

pub fn buff(cmd: &str, app: &mut App) -> command::Result<()> {
    let mut split = cmd.split(" ");
    let _first = split
        .next()
        .expect("Command should always have a first value");

    match split.next() {
        Some("add") | Some("a") => add(&mut split, app)?,
        Some("remove") | Some("delete") | Some("r") | Some("d") => remove(&mut split, app)?,
        _ => return Err(CommandError::ArgumentError),
    };

    Ok(())
}

fn add(split: &mut Split<&str>, app: &mut App) -> command::Result<()> {
    let mut duration = None;
    let mut name = None;

    if let Some(n) = split.next() {
        if let Ok(n) = n.parse::<i32>() {
            duration = Some(n);
        } else {
            name = Some(n);
        }
    }

    if let Some(n2) = split.next() {
        if let Ok(n2) = n2.parse::<i32>() {
            duration = Some(n2);
        } else {
            name = Some(n2);
        }
    }

    let Some(name) = name else {
        return Err(command::CommandError::ArgumentError);
    };

    let cond = Condition {
        name: name.to_string(),
        duration: duration.unwrap_or(-1),
    };

    app.sheet.conditions.conditions.push(cond);

    Ok(())
}

fn remove(split: &mut Split<&str>, app: &mut App) -> command::Result<()> {
    if let Some(remove) = split.next() {
        let data = app
            .sheet
            .conditions
            .conditions
            .iter()
            .map(|f| f.name.as_str())
            .collect::<Vec<&str>>();

        if let filter = search::filter(remove, &data, 2)
            && filter.len() == 1
        {
            app.sheet.conditions.conditions.remove(filter[0]);
            Ok(())
        } else {
            Err(CommandError::ArgumentError)
        }
    } else {
        Err(CommandError::ArgumentError)
    }
}
