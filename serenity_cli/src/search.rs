use std::sync::Arc;

use edit_distance::edit_distance;
use enumflags2::bitflags;
use serenity_types::database::{
    condition::Condition, generic::Generic, spell::Spell, traits::Trait,
};

use crate::data::Data;

const SPELL_THRESHHOLD: usize = 3;
const CONDITION_THERSHOLD: usize = 4;
const TRAIT_THERSHOLD: usize = 3;

pub fn search_spells<'a>(term: &str, spells: &'a Arc<[Spell]>) -> Vec<&'a Spell> {
    let data = spells
        .iter()
        .map(|f| f.index.replace("-", " "))
        .collect::<Vec<String>>();
    let data = data.iter().map(|f| f.as_str()).collect::<Vec<&str>>();
    let elements = filter(term, &data, SPELL_THRESHHOLD);

    let mut results = vec![];
    for i in elements {
        results.push(&spells[i])
    }

    results
}

pub fn search_traits<'a>(term: &str, traits: &'a Arc<[Trait]>) -> Vec<&'a Trait> {
    let data = traits
        .iter()
        .map(|f| f.index.replace("-", " "))
        .collect::<Vec<String>>();
    let data = data.iter().map(|f| f.as_str()).collect::<Vec<&str>>();
    let elements = filter(term, &data, TRAIT_THERSHOLD);

    let mut results = vec![];
    for i in elements {
        results.push(&traits[i])
    }

    results
}

pub fn search_generic<'a>(
    term: &str,
    input: &'a Arc<[Generic]>,
    threshold: usize,
) -> Vec<&'a Generic> {
    let data = input
        .iter()
        .map(|f| f.name.to_lowercase().replace("-", " "))
        .collect::<Vec<String>>();
    let data = data.iter().map(|f| f.as_str()).collect::<Vec<&str>>();
    let elements = filter(term, &data, threshold);

    let mut results = vec![];
    for i in elements {
        results.push(&input[i])
    }

    results
}

pub fn search_conditions<'a>(term: &str, conditions: &'a Arc<[Condition]>) -> Vec<&'a Condition> {
    let mut out = vec![];

    let data = conditions
        .iter()
        .map(|f| f.index.as_str())
        .collect::<Vec<&str>>();

    let elements = filter(term, &data, CONDITION_THERSHOLD);

    for i in elements {
        out.push(&conditions[i]);
    }

    out
}

pub fn filter<'a>(term: &str, data: &[&str], threshold: usize) -> Vec<usize> {
    let mut out = vec![];

    let fuzzy = data
        .iter()
        .enumerate()
        .filter(|(_i, f)| edit_distance(**f, term) < threshold)
        .map(|f| (f.0, *f.1))
        .collect::<Vec<(usize, &str)>>();

    let mut binding = term.split(" ");
    let first = binding.next().expect("Search should always have one word");

    let fuzzy_first = data
        .iter()
        .enumerate()
        .filter(|(_i, f)| edit_distance(&&f, &first) < threshold)
        .map(|f| (f.0, *f.1))
        .collect::<Vec<(usize, &str)>>();

    let exact_first = data
        .iter()
        .enumerate()
        .filter(|(_i, f)| f.to_lowercase().starts_with(&term))
        .map(|f| (f.0, *f.1))
        .collect::<Vec<(usize, &str)>>();

    let any_first_exact = data
        .iter()
        .enumerate()
        .filter(|(_i, f)| f.split(" ").any(|f| f.to_lowercase().starts_with(&term)))
        .map(|(i, f)| (i, *f))
        .collect::<Vec<(usize, &str)>>();

    let any_first_fuzzy = data
        .iter()
        .enumerate()
        .filter(|f| {
            let s = f.1;
            for i in s.split(" ") {
                if edit_distance(i, first) < threshold {
                    return true;
                } else {
                    return false;
                }
            }
            false
        })
        .map(|(i, f)| (i, *f))
        .collect::<Vec<(usize, &str)>>();

    let mut f = exact_first.iter().map(|f| f.0).collect();
    out.append(&mut f);

    let mut f = fuzzy.iter().map(|f| f.0).collect();
    out.append(&mut f);

    let mut f = fuzzy_first.iter().map(|f| f.0).collect();
    out.append(&mut f);

    let mut f = any_first_exact.iter().map(|f| f.0).collect();
    out.append(&mut f);

    let mut f = any_first_fuzzy.iter().map(|f| f.0).collect();
    out.append(&mut f);

    use itertools::Itertools;

    let out = out.iter().unique().copied().collect();

    out
}

pub fn search<'a>(
    term: &str,
    data: &'a Data,
    filters: enumflags2::BitFlags<Filters>,
) -> Vec<String> {
    let mut out = vec![];

    let term = term.to_lowercase();

    if filters.contains(Filters::MagicItems) {
        let items = search_generic(&term, &data.magic_items, 3);

        let mut items = items.iter().map(|f| f.to_string()).collect::<Vec<String>>();

        out.append(&mut items);
    }

    if filters.contains(Filters::Conditions) {
        let conditions = search_conditions(&term, &data.conditions);

        let mut conditions = conditions
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>();

        out.append(&mut conditions);
    }

    if filters.contains(Filters::Spells) {
        let spells = search_spells(&term, &data.spells);

        let mut spells = spells
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>();

        out.append(&mut spells);
    }

    if filters.contains(Filters::Rules) {
        let rules = search_generic(&term, &data.rules, 3);

        let mut rules = rules.iter().map(|f| f.to_string()).collect::<Vec<String>>();

        out.append(&mut rules);
    }

    if filters.contains(Filters::Traits) {
        let traits = search_traits(&term, &data.traits);
        let mut traits = traits
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>();

        out.append(&mut traits);
    }

    if filters.contains(Filters::Skills) {
        let skills = search_generic(&term, &data.skills, 2);

        let mut skills = skills
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>();

        out.append(&mut skills);
    }

    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[bitflags(default = Spells | Conditions | Rules)]
#[repr(u8)]
pub enum Filters {
    Spells,
    Conditions,
    MagicItems,
    Traits,
    Rules,
    Skills,
}
