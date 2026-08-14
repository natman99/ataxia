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
    AcBonus(i64),
    /// Add a spell.
    Spell(Spell),
    /// Add a meter. e.g. spell slots or other limited resources.
    Meter(Meter),
    /// Bonus max health that is applied per level.
    HealthBonusPerLevel(i64),
    /// Ability score bonus (raw stat bonus).
    AbilityScoreBonus(AbilityScoreBonus),
    /// An extra sense type.
    Sense(Sense),
    /// Proficiency in a skill.
    AddProficiency(Skill),
    /// Bonus to initiative.
    InitiativeBonus(i64),
    /// A condition effect for the conditions page
    Condition(Condition),
    Expertise(Skill),
}

impl Effect {
    pub fn apply(&self, s: &mut Character) {
        match &self {
            Effect::AbilityScoreBonus(bonus) => s
                .ability_scores
                .get_mut(&bonus.score)
                .add_bonus(bonus.bonus.into()),
            Effect::AcBonus(b) => {
                let i = s.armor_class.0 + b;
                // Negative bonus should never be bigger than 10
                s.armor_class.0 = i;
            }
            Effect::Spell(spell) => {
                s.spells.spells.insert(spell.name.clone(), spell.clone());
            }
            Effect::Meter(meter) => {
                s.meters
                    .meters
                    .insert(meter.name.to_string(), meter.clone());
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
            Effect::Condition(e) => s.conditions.conditions.push(e.clone()),
            Effect::Expertise(skill) => s.skills.expertise.insert(*skill),
        }
    }
}

impl Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Effect::AcBonus(a) => format!("AC: {a:+}"),
            Effect::Spell(spell) => format!("Spell {}", spell.name),
            Effect::Meter(meter) => format!("Meter {}", meter.name),
            Effect::HealthBonusPerLevel(h) => format!("Hp/lvl {h}"),
            Effect::AbilityScoreBonus(ability_score_bonus) => format!(
                "Score {} {:+}",
                ability_score_bonus.score, ability_score_bonus.bonus
            ),
            Effect::Sense(sense) => format!("{sense}"),
            Effect::AddProficiency(skill) => format!("{skill}"),
            Effect::InitiativeBonus(b) => format!("Initiative {:+}", b),
            Effect::Condition(condition) => format!("{}", condition.name),
            Effect::Expertise(skill) => format!("Expertise {skill}"),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
pub struct Features {
    pub inner: Vec<Feature>,
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
