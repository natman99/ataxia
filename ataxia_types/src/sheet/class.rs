use std::{
    fmt::Display,
    ops::{Deref, Index, IndexMut},
};

use rand::RngExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AbilityScores, HitPoints, roll::Die};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct Classes {
    pub classes: Vec<Class>,
}

impl Classes {
    pub fn heal(
        &mut self,
        index: usize,
        roll: bool,
        health: &mut HitPoints,
        ability_scores: &AbilityScores,
    ) {
        self[index].heal(roll, health, ability_scores);
    }

    pub fn heal_first(
        &mut self,
        roll: bool,
        health: &mut HitPoints,
        ability_scores: &AbilityScores,
    ) -> Option<()> {
        if let Some(class) = self
            .classes
            .iter_mut()
            .find(|f| f.healing_die_remaining > 0)
        {
            class.heal(roll, health, ability_scores);
            Some(())
        } else {
            None
        }
    }
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

impl IndexMut<usize> for Classes {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.classes[index]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct Class {
    pub class: ClassType,
    pub level: Level,
    pub hit_dice: Die,
    pub health_bonus_per_level: i32,
    pub healing_die_remaining: u32,
}

impl Default for Class {
    fn default() -> Self {
        let class: ClassType = Default::default();
        let hit_dice = class.get_hit_dice();
        Self {
            class,
            level: Default::default(),
            hit_dice,
            health_bonus_per_level: 0,
            healing_die_remaining: 1,
        }
    }
}

impl Class {
    pub fn heal(&mut self, roll: bool, health: &mut HitPoints, ability_scores: &AbilityScores) {
        if self.healing_die_remaining == 0 {
            return;
        }
        if roll {
            let mut rng = rand::rng();
            let healing_amount =
                rng.random_range(1..=self.hit_dice.max()) as i32 + ability_scores.con.modifier();
            health.heal(healing_amount);
        }
        self.healing_die_remaining = self.healing_die_remaining.saturating_sub(1);
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
    #[serde(untagged)]
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
