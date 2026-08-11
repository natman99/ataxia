#[cfg(feature = "rhai")]
use rhai::CustomType;
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
pub mod saving_throws;
pub mod senses;
pub mod source;

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
    saving_throws::SavingThrows,
    senses::Senses,
    sheet::{class::ClassType, skills::Skills, spells::Spells},
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]

pub struct Character {
    pub name: String,
    pub race: String,
    pub classes: Classes,
    pub initiative: Initiative,
    pub armor_class: ArmorClass,
    pub health: HitPoints,
    pub inventory: Inventory,
    pub ability_scores: AbilityScores,
    pub skills: Skills,
    pub saving_throws: SavingThrows,
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
        level: i64,
        health: HitPoints,
        class: ClassType,
        ability_scores: AbilityScores,
    ) -> Self {
        Self {
            name,
            health,
            classes: Classes {
                classes: vec![Class {
                    level: Level(level),
                    hit_dice: class.get_hit_dice(),
                    health_bonus_per_level: 0,
                    class,
                    healing_die_remaining: level,
                    subclass: String::new(),
                    max_healing_die: level,
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
            race: "Elf".to_string(),
            lore: Default::default(),
            walking_speed: Default::default(),
            languages: Default::default(),
            conditions: Default::default(),
            ability_scores,
            saving_throws: Default::default(),
        }
    }

    pub fn ability_modifier(&self) -> i64 {
        self.ability_scores.get(&self.ability_modifier).modifier()
    }
    /// Attack bonus.
    /// Ability modifier + proficiency bonus.
    pub fn attack_bonus(&self) -> i32 {
        let f = self.ability_scores.get(&self.ability_modifier).modifier();
        self.skills.proficiency_bonus as i32 + f as i32
    }
    /// Spell attack bonus. Alias for `self.attack_bonus()`
    pub fn spell_bonus(&self) -> i32 {
        self.attack_bonus()
    }
    pub fn save_dc(&self) -> i64 {
        let a = self.ability_scores.get(&self.ability_modifier);
        8 + self.skills.proficiency_bonus + a.modifier()
    }

    pub fn initiative(&self) -> i64 {
        self.initiative.get(&self.ability_scores)
    }

    pub fn passive_senses(&self) -> PassiveSenses {
        PassiveSenses {
            insight: self.senses.get_insight(&self.ability_scores),
            perception: self.senses.get_perception(&self.ability_scores),
            investigation: self.senses.get_perception(&self.ability_scores),
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "name".to_string(),
            race: "Elf".to_string(),
            classes: Default::default(),
            initiative: Default::default(),
            armor_class: Default::default(),
            health: Default::default(),
            inventory: Default::default(),
            ability_scores: Default::default(),
            skills: Default::default(),
            saving_throws: Default::default(),
            spells: Default::default(),
            senses: Default::default(),
            features: Default::default(),
            ability_modifier: Default::default(),
            meters: Default::default(),
            lore: Default::default(),
            walking_speed: Default::default(),
            languages: Default::default(),
            conditions: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PassiveSenses {
    pub insight: i64,
    pub perception: i64,
    pub investigation: i64,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
/// An inventory containing a hash map of `Item`s
pub struct Inventory(pub HashMap<String, Item>);

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
/// Ability scores. Defaults to ten.
pub struct AbilityScores {
    #[cfg_attr(feature = "rhai", rhai_type(get = Self::get_str, set = Self::set_str))]
    pub str: AbilityScore,

    #[cfg_attr(feature = "rhai", rhai_type(get = Self::get_dex, set = Self::set_dex))]
    pub dex: AbilityScore,

    #[cfg_attr(feature = "rhai", rhai_type(get = Self::get_con, set = Self::set_con))]
    pub con: AbilityScore,

    #[cfg_attr(feature = "rhai", rhai_type(get = Self::get_int, set = Self::set_int))]
    pub int: AbilityScore,

    #[cfg_attr(feature = "rhai", rhai_type(get = Self::get_wis, set = Self::set_wis))]
    pub wis: AbilityScore,

    #[cfg_attr(feature = "rhai", rhai_type(get = Self::get_cha, set = Self::set_cha))]
    pub cha: AbilityScore,
}

impl AbilityScores {
    /// Collect all scores into an iter
    pub fn iter(&self) -> [&AbilityScore; 6] {
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

    pub fn set_bonus(&mut self, score: &Score, num: i64) {
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

    pub fn get_str(&self) -> i64 {
        self.str.base
    }

    pub fn set_str(&mut self, new: i64) {
        self.str.base = new;
    }

    pub fn get_dex(&self) -> i64 {
        self.dex.base
    }

    pub fn set_dex(&mut self, new: i64) {
        self.dex.base = new;
    }

    pub fn get_con(&self) -> i64 {
        self.con.base
    }

    pub fn set_con(&mut self, new: i64) {
        self.con.base = new;
    }

    pub fn get_int(&self) -> i64 {
        self.int.base
    }

    pub fn set_int(&mut self, new: i64) {
        self.int.base = new;
    }

    pub fn get_wis(&self) -> i64 {
        self.wis.base
    }

    pub fn set_wis(&mut self, new: i64) {
        self.wis.base = new;
    }

    pub fn get_cha(&self) -> i64 {
        self.cha.base
    }

    pub fn set_cha(&mut self, new: i64) {
        self.cha.base = new;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, EnumString, Hash, Eq)]
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

impl Score {
    pub const ALL: [Self; 6] = [
        Self::Str,
        Self::Dex,
        Self::Con,
        Self::Int,
        Self::Wis,
        Self::Cha,
    ];

    pub const ALL_STR: [&str; 6] = ["str", "dex", "con", "int", "wis", "cha"];
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
#[cfg_attr(feature = "rhai", derive(CustomType))]
/// The armor class of a character.
pub struct ArmorClass(
    #[cfg_attr(feature = "rhai", rhai_type(set = Self::set, get = Self::get))] pub i64,
);

impl ArmorClass {
    /// 10 + dex
    fn new(scores: &AbilityScores) -> Self {
        // safety: negative modifier can never be above 10.
        Self(10 + scores.dex.modifier())
    }

    pub fn get(&self) -> i64 {
        self.0
    }

    pub fn set(&mut self, s: i64) {
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
#[cfg_attr(feature = "rhai", derive(CustomType))]
/// Initiative score.
#[derive(Default)]
pub struct Initiative {
    pub bonus: i64,
}

impl Initiative {
    pub fn get(&self, scores: &AbilityScores) -> i64 {
        scores.get(&Score::Dex).modifier() + self.bonus
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
/// Walking speed in feet.
pub struct WalkingSpeed(pub i64);

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
