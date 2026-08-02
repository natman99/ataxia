use std::fmt::Display;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "rhai")]
use rhai::CustomType;

pub use feature::HasFeature;

mod feature;

use crate::{
    Character, Score,
    condition::Condition,
    meter::Meter,
    senses::Sense,
    sheet::source::{HasSource, Source},
    skills::Skill,
    spells::Spell,
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
pub struct Feature {
    pub source: Option<Source>,
    pub effects: Vec<Effect>,
}

impl HasSource for Feature {
    fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    fn add_source(&mut self, source: Source) {
        self.source = Some(source)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
/// The effect the feature has
pub enum Effect {
    /// Flat armor class (ac) bonus.
    AcBonus(i32),
    /// Add a spell.
    Spell(Spell),
    /// Add a meter. e.g. spell slots or other limited resources.
    Meter(MeterAdd),
    /// Bonus max health that is applied per level.
    HealthBonusPerLevel(i32),
    /// Ability score bonus (raw stat bonus).
    AbilityScoreBonus(AbilityScoreBonus),
    /// An extra sense type.
    Sense(Sense),
    /// Proficiency in a skill.
    AddProficiency(Skill),
    /// Bonus to initiative.
    InitiativeBonus(i32),
    /// An effect for the effects page
    Effect(Condition),
    Expertise(Skill),
}

impl Effect {
    pub fn apply(&self, s: &mut Character) {
        match &self {
            Effect::AbilityScoreBonus(bonus) => s
                .ability_scores
                .get_mut(&bonus.score)
                .add_bonus(bonus.bonus),
            Effect::AcBonus(b) => {
                let i = s.armor_class.0 as i32 + b;
                // Negative bonus should never be bigger than 10
                s.armor_class.0 = i as u32;
            }
            Effect::Spell(spell) => {
                s.spells.spells.insert(spell.name.clone(), spell.clone());
            }
            Effect::Meter(meter_add) => {
                s.meters
                    .meters
                    .insert(meter_add.name.to_string(), meter_add.meter.clone());
            }
            Effect::HealthBonusPerLevel(b) => {
                s.health.level_bonus += b;
            }
            Effect::Sense(sense) => match sense {
                Sense::BlindSight => s.senses.extra.blind_sight = true,
                Sense::DarkVision => s.senses.extra.dark_vision = true,
                Sense::TremorSense => s.senses.extra.tremor_sense = true,
                Sense::TrueSight => s.senses.extra.true_sight = true,
            },
            Effect::AddProficiency(skill) => {
                s.skills.proficiencies.set(*skill, true);
            }
            Effect::InitiativeBonus(b) => s.initiative.bonus += b,
            Effect::Effect(e) => s.conditions.conditions.push(e.clone()),
            Effect::Expertise(skill) => s.skills.expertise.insert(*skill),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
pub struct Features {
    pub inner: Vec<Feature>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct MeterAdd {
    pub name: String,
    pub meter: Meter,
    /// Number of meter slots.
    pub slot_number: usize,
}

#[derive(Debug, Clone, PartialEq, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct AbilityScoreBonus {
    /// The abilty score to boost.
    pub score: Score,
    /// The amount to increase by.
    pub bonus: i32,
}

impl Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::AcBonus(b) => write!(f, "AC {b:+}"),
            Effect::Spell(spell) => write!(f, "Spell: {}", spell.name),
            Effect::Meter(meter_add) => write!(f, "{}", meter_add.name),
            Effect::HealthBonusPerLevel(b) => write!(f, "Health: {b:+}"),
            Effect::AbilityScoreBonus(ability_score_bonus) => write!(
                f,
                "{} {:+}",
                ability_score_bonus.score, ability_score_bonus.bonus
            ),
            Effect::Sense(sense) => write!(f, "{sense}"),
            Effect::AddProficiency(skill) => write!(f, "Proficiency: {}", skill),
            Effect::InitiativeBonus(b) => write!(f, "Initiative {b:+}"),
            Effect::Effect(condition) => write!(f, "Condition: {}", condition.name),
            Effect::Expertise(skill) => write!(f, "Expertise: {skill}"),
        }
    }
}
