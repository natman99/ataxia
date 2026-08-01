#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};
use strum::EnumString;

pub mod ability_score;
pub mod class;
mod hit_points;
mod item;
pub mod roll;
pub mod skills;
pub mod spells;

pub mod condition;
pub mod damage;
pub mod feat;
pub mod feature;
pub mod language;
pub mod lore;
pub mod meter;
pub mod senses;
mod source;

pub use hit_points::HitPoints;
pub use item::Item;

use ability_score::AbilityScore;

use crate::{
    class::{Class, Classes, Level},
    condition::Conditions,
    feature::Features,
    language::Languages,
    lore::Lore,
    meter::Meters,
    senses::Senses,
    sheet::{class::ClassType, skills::Skills, spells::Spells},
};

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Character {
    pub name: String,
    pub race: String,
    pub class: Classes,
    pub initiative: Initiative,
    pub armor_class: ArmorClass,
    pub health: HitPoints,
    pub inventory: Inventory,
    pub ability_scores: AbilityScores,
    pub skills: Skills,
    pub spells: Spells,
    pub senses: Senses,
    pub features: Features,
    pub ability_modifier: Score,
    pub meters: Meters,
    pub lore: Lore,
    pub walking_speed: WalkingSpeed,
    pub languages: Languages,
    pub conditions: Conditions,
}

impl Character {
    pub fn new(
        name: String,
        level: u32,
        health: HitPoints,
        class: ClassType,
        ability_scores: AbilityScores,
    ) -> Self {
        Self {
            name,
            health,
            class: Classes {
                classes: vec![Class {
                    level: Level(level),
                    hit_dice: class.get_hit_dice(),
                    health_bonus_per_level: 0,
                    class,
                    healing_die_remaining: level,
                    subclass: String::new(),
                }],
            },
            skills: Skills::default(),
            inventory: Default::default(),
            spells: Default::default(),
            senses: Default::default(),
            initiative: Default::default(),
            armor_class: ArmorClass::new(&ability_scores),
            ability_modifier: Score::Str,
            meters: Default::default(),
            features: Default::default(),
            race: Default::default(),
            lore: Default::default(),
            walking_speed: Default::default(),
            languages: Default::default(),
            conditions: Default::default(),
            ability_scores,
        }
    }

    pub fn ability_modifier(&self) -> i32 {
        self.ability_scores.get(&self.ability_modifier).modifier()
    }
    /// Attack bonus.
    /// Ability modifier + proficiency bonus.
    pub fn attack_bonus(&self) -> i32 {
        let f = self.ability_scores.get(&self.ability_modifier).modifier();
        self.skills.proficiency_bonus as i32 + f
    }
    /// Spell attack bonus. Alias for `self.attack_bonus()`
    pub fn spell_bonus(&self) -> i32 {
        self.attack_bonus()
    }
    pub fn save_dc(&self) -> i32 {
        let a = self.ability_scores.get(&self.ability_modifier);
        8 + self.skills.proficiency_bonus as i32 + a.modifier()
    }

    pub fn initiative(&self) -> i32 {
        self.initiative.get(&self.ability_scores)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
/// An inventory containing a hash map of `Item`s
pub struct Inventory(pub HashMap<String, Item>);

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
/// Ability scores. Defaults to ten.
pub struct AbilityScores {
    pub str: AbilityScore,
    pub dex: AbilityScore,
    pub con: AbilityScore,
    pub int: AbilityScore,
    pub wis: AbilityScore,
    pub cha: AbilityScore,
}

impl AbilityScores {
    /// Collect all scores into an iter
    pub fn iter<'a>(&'a self) -> [&'a AbilityScore; 6] {
        [
            &self.str, &self.dex, &self.con, &self.int, &self.wis, &self.cha,
        ]
    }

    pub fn get(&self, score: &Score) -> &AbilityScore {
        match score {
            Score::Str => &self.str,
            Score::Dex => &self.dex,
            Score::Con => &self.con,
            Score::Int => &self.int,
            Score::Wis => &self.wis,
            Score::Cha => &self.cha,
        }
    }

    pub fn get_mut(&mut self, score: &Score) -> &mut AbilityScore {
        match score {
            Score::Str => &mut self.str,
            Score::Dex => &mut self.dex,
            Score::Con => &mut self.con,
            Score::Int => &mut self.int,
            Score::Wis => &mut self.wis,
            Score::Cha => &mut self.cha,
        }
    }

    pub fn set_bonus(&mut self, score: &Score, num: i32) {
        match score {
            Score::Str => self.str.set_bonus(num),
            Score::Dex => self.dex.set_bonus(num),
            Score::Con => self.con.set_bonus(num),
            Score::Int => self.int.set_bonus(num),
            Score::Wis => self.wis.set_bonus(num),
            Score::Cha => self.cha.set_bonus(num),
        }
    }

    /// Reset all bonuses.
    pub fn reset(&mut self) {
        self.str.reset();
        self.dex.reset();
        self.con.reset();
        self.int.reset();
        self.cha.reset();
        self.wis.reset();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, EnumString)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[strum(ascii_case_insensitive)]
/// Scores enum.
pub enum Score {
    #[default]
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Score::Str => "Str",
            Score::Dex => "Dex",
            Score::Con => "Con",
            Score::Int => "Int",
            Score::Wis => "Wis",
            Score::Cha => "Cha",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
/// The armor class of a character.
pub struct ArmorClass(u32);

impl ArmorClass {
    /// 10 + dex
    fn new(scores: &AbilityScores) -> Self {
        // safety: negative modifier can never be above 10.
        Self((10 + scores.dex.modifier()) as u32)
    }

    pub fn get(&self) -> u32 {
        self.0
    }

    pub fn set(&mut self, s: u32) {
        self.0 = s;
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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
/// Initiative score.
pub struct Initiative {
    pub bonus: i32,
}

impl Default for Initiative {
    fn default() -> Self {
        Self { bonus: 0 }
    }
}

impl Initiative {
    pub fn get(&self, scores: &AbilityScores) -> i32 {
        scores.get(&Score::Dex).modifier() + self.bonus
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
/// Walking speed in feet.
pub struct WalkingSpeed(u32);

impl Default for WalkingSpeed {
    fn default() -> Self {
        Self(30)
    }
}

impl Display for WalkingSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ft", &self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_scores() {
        Score::from_str("str").unwrap();
        Score::from_str("Str").unwrap();
    }
}
