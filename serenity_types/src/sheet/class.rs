use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(from = "String")]
/// The class of a character.
pub enum Class {
    Fighter,
    Wizard,
    Sorcerer,
    Cleric,
    Barbarian,
    Monk,
    Ranger,
    Paladin,
    Rougue,
    Warlock,
    Bard,
    Druid,
    Other(String),
}

impl From<String> for Class {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "fighter" => Class::Fighter,
            "wizard" => Class::Wizard,
            "sorcerer" => Class::Sorcerer,
            "cleric" => Class::Cleric,
            "barbarian" => Class::Barbarian,
            "monk" => Class::Monk,
            "ranger" => Class::Ranger,
            "paladin" => Class::Paladin,
            "rougue" => Class::Rougue,
            "warlock" => Class::Warlock,
            "bard" => Class::Bard,
            "druid" => Class::Druid,
            _ => Class::Other(value),
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = match &self {
            Class::Fighter => "Fighter",
            Class::Wizard => "Wizard",
            Class::Sorcerer => "Sorcerer",
            Class::Cleric => "Cleric",
            Class::Barbarian => "Barbarian",
            Class::Monk => "Monk",
            Class::Ranger => "Ranger",
            Class::Paladin => "Paladin",
            Class::Rougue => "Rougue",
            Class::Warlock => "Warlock",
            Class::Bard => "Bard",
            Class::Druid => "Druid",
            Class::Other(name) => name.as_str(),
        };

        write!(f, "{s}")
    }
}

impl Default for Class {
    fn default() -> Self {
        Class::Fighter
    }
}
