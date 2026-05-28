use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(default)]
pub struct Languages {
    languages: Vec<Language>,
}

impl Display for Languages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .languages
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{s}")
    }
}

impl Default for Languages {
    fn default() -> Self {
        Self {
            languages: vec![Language::Common],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hash)]
pub enum Language {
    Common,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    Orc,
    Abyssal,
    Celestial,
    Draconic,
    DeepSpeech,
    Infernal,
    Primordial,
    Sylvan,
    Undercommon,
    ThievesCant,
    Other(String),
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Language::Common => "Common",
            Language::Dwarvish => "Dwarvish",
            Language::Elvish => "Elvish",
            Language::Giant => "Giant",
            Language::Gnomish => "Gnomish",
            Language::Goblin => "Goblin",
            Language::Halfling => "Halfling",
            Language::Orc => "Orc",
            Language::Abyssal => "Abyssal",
            Language::Celestial => "Celestial",
            Language::Draconic => "Draconic",
            Language::DeepSpeech => "Deep Speech",
            Language::Infernal => "Infernal",
            Language::Primordial => "Primordial",
            Language::Sylvan => "Sylvan",
            Language::Undercommon => "Undercommon",
            Language::ThievesCant => "Thieves' Cant",
            Language::Other(name) => &format!("{}*", name),
        };

        write!(f, "{}", s)
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "common" => Language::Common,
            "dwarvish" => Language::Dwarvish,
            "elvish" => Language::Elvish,
            "giant" => Language::Giant,
            "gnomish" => Language::Gnomish,
            "goblin" => Language::Goblin,
            "halfling" => Language::Halfling,
            "orc" => Language::Orc,
            "abyssal" => Language::Abyssal,
            "celestial" => Language::Celestial,
            "draconic" => Language::Draconic,
            "deep speech" => Language::DeepSpeech,
            "infernal" => Language::Infernal,
            "primordial" => Language::Primordial,
            "sylvan" => Language::Sylvan,
            "undercommon" => Language::Undercommon,
            "thieves' cant" => Language::ThievesCant,
            e => Language::Other(e.to_string()),
        }
    }
}
