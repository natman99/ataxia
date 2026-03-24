use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct Lore {
    backstory: String,
    personality_traits: String,
    allies: String,
    enemies: String,
    physical_traits: String,
    alignment: Alignment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct Alignment {
    morality: Morality,
    order: Order,
}

impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.order, self.morality)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
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
