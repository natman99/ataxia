use std::{
    fmt::Display,
    ops::{Deref, Index},
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::roll::Die;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Classes {
    pub classes: Vec<Class>,
}

impl Default for Classes {
    fn default() -> Self {
        let class = Class::default();
        Self {
            classes: vec![class],
        }
    }
}

impl Display for Classes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in &self.classes {
            s.push_str(&format!("{} {} ", i.class, i.level));
        }
        write!(f, "{}", s)
    }
}

impl Deref for Classes {
    type Target = Vec<Class>;

    fn deref(&self) -> &Self::Target {
        &self.classes
    }
}

impl Index<usize> for Classes {
    type Output = Class;

    fn index(&self, index: usize) -> &Self::Output {
        &self.classes[index]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Class {
    pub class: ClassType,
    pub level: Level,
    pub hit_dice: Die,
    pub health_bonus_per_level: i32,
}

impl Default for Class {
    fn default() -> Self {
        Self {
            class: Default::default(),
            level: Default::default(),
            hit_dice: Default::default(),
            health_bonus_per_level: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(from = "String")]
/// The class of a character.
pub enum ClassType {
    #[default]
    Fighter,
    Wizard,
    Sorcerer,
    Cleric,
    Barbarian,
    Monk,
    Ranger,
    Paladin,
    Rogue,
    Warlock,
    Bard,
    Druid,
    Other(String),
}

impl From<String> for ClassType {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "fighter" => ClassType::Fighter,
            "wizard" => ClassType::Wizard,
            "sorcerer" => ClassType::Sorcerer,
            "cleric" => ClassType::Cleric,
            "barbarian" => ClassType::Barbarian,
            "monk" => ClassType::Monk,
            "ranger" => ClassType::Ranger,
            "paladin" => ClassType::Paladin,
            "rogue" => ClassType::Rogue,
            "warlock" => ClassType::Warlock,
            "bard" => ClassType::Bard,
            "druid" => ClassType::Druid,
            _ => ClassType::Other(value),
        }
    }
}

impl ClassType {
    pub fn get_hit_dice(&self) -> Die {
        match self {
            ClassType::Fighter => Die::D10,
            ClassType::Wizard => Die::D6,
            ClassType::Sorcerer => Die::D6,
            ClassType::Cleric => Die::D8,
            ClassType::Barbarian => Die::D12,
            ClassType::Monk => Die::D8,
            ClassType::Ranger => Die::D10,
            ClassType::Paladin => Die::D10,
            ClassType::Rogue => Die::D8,
            ClassType::Warlock => Die::D8,
            ClassType::Bard => Die::D8,
            ClassType::Druid => Die::D8,
            ClassType::Other(_) => Die::D8,
        }
    }
}

impl Display for ClassType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = match &self {
            ClassType::Fighter => "Fighter",
            ClassType::Wizard => "Wizard",
            ClassType::Sorcerer => "Sorcerer",
            ClassType::Cleric => "Cleric",
            ClassType::Barbarian => "Barbarian",
            ClassType::Monk => "Monk",
            ClassType::Ranger => "Ranger",
            ClassType::Paladin => "Paladin",
            ClassType::Rogue => "Rogue",
            ClassType::Warlock => "Warlock",
            ClassType::Bard => "Bard",
            ClassType::Druid => "Druid",
            ClassType::Other(name) => name.as_str(),
        };

        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Level(pub u32);

impl Default for Level {
    fn default() -> Self {
        Self(1)
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
