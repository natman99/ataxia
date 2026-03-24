use std::collections::{BTreeMap, HashMap};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    Score,
    ability_score::AbilityScore,
    database::spell::Spell,
    meter::Meter,
    roll::{Die, Roll},
    senses::Sense,
    skills::Skill,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub roll: Option<Roll>,
    pub effects: Vec<FeatureEffect>,
}

impl Default for Feature {
    fn default() -> Self {
        Self {
            name: "example spell".to_string(),
            description: "i explode something".to_string(),
            roll: Some(Roll::new(&[Die::D20], 0)),
            effects: vec![],
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, JsonSchema)]
pub struct Features {
    pub inner: BTreeMap<String, Feature>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
/// The effect the feature has
pub enum FeatureEffect {
    /// Flat armor class (ac) bonus.
    AcBonus(i32),
    /// Add a spell.
    Spell(Spell),
    /// Add a meter. e.g. spell slots or other limited resources.
    Meter(MeterAdd),
    // /// Flat bonus health.
    // HealthBonus(u32),
    /// Bonus max health that is applied per level.
    HealthBonusPerLevel(i32),
    /// Ability score bonus (raw stat bonus).
    AbilityScoreBonus(AbilityScoreBonus),
    /// An extra sense type.
    Sense(Sense),
    /// Proficiency in a skill.
    AddProficiency(Skill),
    /// Bonus to initiative.
    InitiativeBonus(u32),

    Nothing,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct MeterAdd {
    pub meter: Meter,
    /// Number of meter slots.
    pub slot_number: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, Copy)]
pub struct AbilityScoreBonus {
    pub score: Score,
    pub bonus: u32,
}
