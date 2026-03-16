use enumflags2::{BitFlags, bitflags};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

pub mod ability_score;
pub mod class;
mod hit_points;
mod item;
pub mod roll;
pub mod skills;
pub mod spells;

pub mod ability;
pub mod r#trait;

pub use hit_points::HitPoints;
pub use item::Item;

use ability_score::AbilityScore;

use crate::{
    ability::Abilities,
    sheet::{class::Class, skills::Skills, spells::Spells},
    r#trait::Features,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Character {
    pub name: String,
    pub level: Level,
    // TODO fix later
    pub initiative: Initiative,
    pub armor_class: ArmorClass,
    pub health: HitPoints,
    pub class: Class,
    pub inventory: Inventory,
    pub ability_scores: AbilityScores,
    pub skills: Skills,
    pub spells: Spells,
    pub senses: Senses,
    pub traits: Features,
    pub abilities: Abilities,
    pub ability_modifier: Score,
}

impl Character {
    pub fn new(
        name: String,
        level: u32,
        health: HitPoints,
        class: Class,
        ability_scores: AbilityScores,
    ) -> Self {
        Self {
            name,
            level: Level(level),
            health,
            class,
            skills: Skills::default(),
            inventory: Default::default(),
            ability_scores,
            spells: Default::default(),
            senses: Default::default(),
            initiative: Default::default(),
            armor_class: Default::default(),
            traits: Default::default(),
            abilities: Default::default(),
            ability_modifier: Score::Str,
        }
    }

    pub fn ability_modifier(&self) -> i32 {
        self.ability_scores.get(&self.ability_modifier).modifier()
    }
    /// Ability modifier + proficiency bonus.
    pub fn attack_bonus(&self) -> i32 {
        let f = self.ability_scores.get(&self.ability_modifier).modifier();
        self.skills.proficiency_bonus as i32 + f
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Inventory(HashMap<String, Item>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
/// Ability scores. Defaults to ten.
pub struct AbilityScores {
    pub str: AbilityScore,
    pub dex: AbilityScore,
    pub con: AbilityScore,
    pub int: AbilityScore,
    pub wis: AbilityScore,
    pub char: AbilityScore,
}

impl AbilityScores {
    /// Collect all scores into an iter
    pub fn iter<'a>(&'a self) -> [&'a AbilityScore; 6] {
        [
            &self.str, &self.dex, &self.con, &self.int, &self.wis, &self.char,
        ]
    }

    pub fn get(&self, score: &Score) -> &AbilityScore {
        match score {
            Score::Str => &self.str,
            Score::Dex => &self.dex,
            Score::Con => &self.con,
            Score::Int => &self.int,
            Score::Wis => &self.wis,
            Score::Char => &self.char,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum Score {
    #[default]
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Char,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArmorClass(u32);

impl ArmorClass {
    pub fn new(ac: u32) -> Self {
        Self(ac)
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

impl Default for ArmorClass {
    fn default() -> Self {
        Self(10)
    }
}

impl Display for ArmorClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Initiative(u32);

impl Display for Initiative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Initiative {
    fn default() -> Self {
        Self(10)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Senses {
    pub perception: i32,
    pub investigation: i32,
    pub insight: i32,
    pub extra: BitFlags<ExtraSenses>,
}

impl Default for Senses {
    fn default() -> Self {
        Self {
            perception: 10,
            investigation: 10,
            insight: 10,
            extra: BitFlags::empty(),
        }
    }
}

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtraSenses {
    BlindSight,
    DarkVision,
    TremorSense,
    TrueSight,
}

impl Display for ExtraSenses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ExtraSenses::BlindSight => "Blindsight",
            ExtraSenses::DarkVision => "Darkvision",
            ExtraSenses::TremorSense => "Tremorsense",
            ExtraSenses::TrueSight => "Truesight",
        };

        write!(f, "{}", s)
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Serialize, Deserialize)]
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
