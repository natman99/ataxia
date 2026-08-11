use std::fmt::Display;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "rhai")]
use rhai::CustomType;

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
pub struct Lore {
    pub backstory: String,
    pub personality_traits: String,
    pub allies: String,
    pub enemies: String,
    pub physical_traits: String,
    pub alignment: Alignment,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
pub struct Alignment {
    pub morality: Morality,
    pub order: Order,
}

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.order, self.morality)
    }
}

#[derive(Debug, Clone, PartialEq, Default, strum::EnumString)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[strum(ascii_case_insensitive)]
pub enum Morality {
    #[default]
    Good,

    Neutral,
    Evil,
}

impl Display for Morality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Morality::Good => "Good ",
            Morality::Neutral => "Neutral ",
            Morality::Evil => "Evil ",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Default, strum::EnumString)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[strum(ascii_case_insensitive)]
pub enum Order {
    #[default]
    Lawful,
    Neutral,
    Chaotic,
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Order::Lawful => "Lawful",
            Order::Neutral => "Neutral",
            Order::Chaotic => "Chaotic",
        };

        write!(f, "{}", s)
    }
}
