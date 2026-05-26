use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

pub mod ability_score;
pub mod class;
mod hit_points;
mod item;
pub mod roll;
pub mod skills;
pub mod spells;

pub mod feature;
pub mod lore;
pub mod meter;
pub mod senses;

pub use hit_points::HitPoints;
pub use item::Item;

use ability_score::AbilityScore;

use crate::{
    class::{Class, Classes, Level},
    feature::{Feature, Features, MeterAdd},
    lore::Lore,
    meter::{Meter, Meters, Metric},
    senses::Senses,
    sheet::{class::ClassType, skills::Skills, spells::Spells},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
#[serde(default)]
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
                }],
            },
            skills: Skills::default(),
            inventory: Default::default(),
            ability_scores,
            spells: Default::default(),
            senses: Default::default(),
            initiative: Default::default(),
            armor_class: Default::default(),
            ability_modifier: Score::Str,
            meters: Default::default(),
            features: Default::default(),
            race: Default::default(),
            lore: Default::default(),
            walking_speed: Default::default(),
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
    /// Calculate the sheet, updating items, health, and stats.
    pub fn calculate(&self) -> Self {
        let mut s = self.clone();
        // reset ability scores.
        s.ability_scores.reset();
        let mut stat_upgrades = vec![];
        // first apply stat changes
        for i in s.features.inner.iter() {
            for f in &i.1.effects {
                match f {
                    feature::FeatureEffect::AbilityScoreBonus(ability_score_bonus) => {
                        stat_upgrades.push(ability_score_bonus)
                    }
                    _ => (),
                }
            }
        }

        for i in stat_upgrades {
            let bonus = s.ability_scores.get(&i.score).get_bonus() + i.bonus;
            s.ability_scores.set_bonus(&i.score, bonus);
        }
        // reset other stats, after stats have been applied
        s.armor_class = ArmorClass::new(&s.ability_scores);
        s.initiative = Initiative::new(&s.ability_scores);

        for i in s.features.inner.iter() {
            for f in &i.1.effects {
                match f {
                    // ignore this the second time
                    feature::FeatureEffect::AbilityScoreBonus(_) => (),
                    feature::FeatureEffect::AcBonus(b) => {
                        let i = s.armor_class.0 as i32 + b;
                        // Negative bonus should never be bigger than 10
                        s.armor_class.0 = i as u32;
                    }
                    feature::FeatureEffect::Spell(spell) => {
                        s.spells.spells.insert(spell.name.clone(), spell.clone());
                    }
                    feature::FeatureEffect::Meter(meter_add) => {
                        s.meters
                            .meters
                            .insert(meter_add.meter.name.clone(), meter_add.meter.clone());
                    }

                    feature::FeatureEffect::HealthBonusPerLevel(b) => {
                        s.health.level_bonus += b;
                    }
                    feature::FeatureEffect::Sense(sense) => match sense {
                        senses::Sense::BlindSight => s.senses.extra.blind_sight = true,
                        senses::Sense::DarkVision => s.senses.extra.dark_vision = true,
                        senses::Sense::TremorSense => s.senses.extra.tremor_sense = true,
                        senses::Sense::TrueSight => s.senses.extra.true_sight = true,
                    },
                    feature::FeatureEffect::AddProficiency(skill) => {
                        s.skills.proficiencies.set(*skill, true);
                    }
                    feature::FeatureEffect::InitiativeBonus(b) => s.initiative.0 += b,
                    feature::FeatureEffect::Nothing => (),
                }
            }
        }

        // do last
        s.health = s.health.fixed(&s.ability_scores, &s.class);
        s
    }

    pub fn save_dc(&self) -> i32 {
        let a = self.ability_scores.get(&self.ability_modifier);
        8 + self.skills.proficiency_bonus as i32 + a.modifier()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
/// An inventory containing a hash map of `Item`s
pub struct Inventory(pub HashMap<String, Item>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
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

    pub fn set_bonus(&mut self, score: &Score, num: u32) {
        match score {
            Score::Str => self.str.set_bonus(num),
            Score::Dex => self.dex.set_bonus(num),
            Score::Con => self.con.set_bonus(num),
            Score::Int => self.int.set_bonus(num),
            Score::Wis => self.wis.set_bonus(num),
            Score::Char => self.char.set_bonus(num),
        }
    }

    /// Reset all bonuses.
    pub fn reset(&mut self) {
        self.str.reset();
        self.dex.reset();
        self.con.reset();
        self.int.reset();
        self.char.reset();
        self.wis.reset();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
/// Scores enum.
pub enum Score {
    #[default]
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Char,
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Score::Str => "Str",
            Score::Dex => "Dex",
            Score::Con => "Con",
            Score::Int => "Int",
            Score::Wis => "Wis",
            Score::Char => "Char",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
/// Initiative score.
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

impl Initiative {
    /// 10 + dex
    fn new(scores: &AbilityScores) -> Self {
        // safety: negative modifier can never be above 10.
        Self((10 + scores.dex.modifier()) as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, JsonSchema)]
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
