use std::{fmt::Display, sync::Arc};

use edit_distance::edit_distance;
use serenity_types::database::{condition::Condition, spell::Spell};

use crate::data::Data;

pub fn search_spells<'a>(term: &str, spells: &'a Arc<[Spell]>) -> Vec<&'a Spell> {
    let data = spells
        .iter()
        .map(|f| f.name.as_str())
        .collect::<Vec<&str>>();
    let elements = filter(term, &data);

    let mut results = vec![];
    for i in elements {
        results.push(&spells[i])
    }

    results
}

pub fn search_conditions<'a>(term: &str, conditions: &'a Arc<[Condition]>) -> Vec<&'a Condition> {
    let mut out = vec![];

    out
}

fn filter<'a>(term: &str, data: &[&str]) -> Vec<usize> {
    let mut out = vec![];
    let fuzzy = data
        .iter()
        .enumerate()
        .filter(|(_i, f)| edit_distance(**f, term) < 2)
        .map(|f| (f.0, *f.1))
        .collect::<Vec<(usize, &str)>>();

    let binding = term.split(" ").collect::<Vec<&str>>();
    let first = binding.first().expect("Search should always have one word");

    let fuzzy_first = data
        .iter()
        .enumerate()
        .filter(|(_i, f)| edit_distance(&&f, &first) < 2)
        .map(|f| (f.0, *f.1))
        .collect::<Vec<(usize, &str)>>();

    let exact_first = data
        .iter()
        .enumerate()
        .filter(|(_i, f)| f.starts_with(term))
        .map(|f| (f.0, *f.1))
        .collect::<Vec<(usize, &str)>>();

    let mut f = exact_first.iter().map(|f| f.0).collect();
    out.append(&mut f);

    let mut f = fuzzy.iter().map(|f| f.0).collect();
    out.append(&mut f);

    let mut f = fuzzy_first.iter().map(|f| f.0).collect();
    out.append(&mut f);
    use itertools::Itertools;

    let out = out.iter().unique().copied().collect();

    out
}

pub fn search<'a, T: Display>(term: &str, data: &'a Data) -> Vec<String> {
    let mut out = vec![];

    let spells = search_spells(term, &data.spells);

    let conditions = search_conditions(term, &data.conditions);
    let mut conditions = conditions
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>();
    out.append(&mut conditions);

    out
}
