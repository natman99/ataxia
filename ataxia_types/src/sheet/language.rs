use std::{fmt::Display, str::FromStr};

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "rhai")]
use rhai::CustomType;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
pub struct Languages {
    pub languages: Vec<Language>,
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

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum Language {
    Common,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    Orcish,
    Abyssal,
    Celestial,
    Draconic,
    DeepSpeech,
    Infernal,
    Primordial,
    Sylvan,
    Undercommon,
    ThievesCant,
    #[cfg_attr(feature = "serde", serde(untagged))]
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
            Language::Orcish => "Orcish",
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

impl From<String> for Language {
    fn from(value: String) -> Self {
        Self::from_str(&value).expect("Cannot fail")
    }
}
impl From<&str> for Language {
    fn from(value: &str) -> Self {
        Self::from_str(value).expect("Cannot fail")
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "common" => Language::Common,
            "dwarvish" => Language::Dwarvish,
            "elvish" => Language::Elvish,
            "giant" => Language::Giant,
            "gnomish" => Language::Gnomish,
            "goblin" => Language::Goblin,
            "halfling" => Language::Halfling,
            "orcish" => Language::Orcish,
            "abyssal" => Language::Abyssal,
            "celestial" => Language::Celestial,
            "draconic" => Language::Draconic,
            "deep speech" => Language::DeepSpeech,
            "infernal" => Language::Infernal,
            "primordial" => Language::Primordial,
            "sylvan" => Language::Sylvan,
            "undercommon" => Language::Undercommon,
            "thieves' cant" | "thieves" | "thieves cant" => Language::ThievesCant,
            e => Language::Other(e.to_string()),
        })
    }
}
