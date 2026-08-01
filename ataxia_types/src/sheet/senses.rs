use std::fmt::Display;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::{AbilityScores, Score};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Senses {
    pub perception_bonus: i32,
    pub investigation_bonus: i32,
    pub insight_bonus: i32,
    pub extra: ExtraSenses,
}

impl Senses {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_perception(&self, scores: &AbilityScores) -> i32 {
        scores.get(&Score::Wis).modifier() + self.perception_bonus
    }

    pub fn get_insight(&self, scores: &AbilityScores) -> i32 {
        scores.get(&Score::Wis).modifier() + self.insight_bonus
    }

    pub fn get_investigation(&self, scores: &AbilityScores) -> i32 {
        scores.get(&Score::Int).modifier() + self.investigation_bonus
    }
}

impl Default for Senses {
    fn default() -> Self {
        Self {
            extra: Default::default(),
            perception_bonus: 0,
            investigation_bonus: 0,
            insight_bonus: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct ExtraSenses {
    pub blind_sight: bool,
    pub dark_vision: bool,
    pub tremor_sense: bool,
    pub true_sight: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
/// Extra sense types.
pub enum Sense {
    BlindSight,
    DarkVision,
    TremorSense,
    TrueSight,
}

impl Display for ExtraSenses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        if self.blind_sight {
            out.push_str("BlindSight ");
        }

        if self.dark_vision {
            out.push_str("DarkVision ");
        }

        if self.tremor_sense {
            out.push_str("TremorSense ");
        }

        if self.true_sight {
            out.push_str("TrueSight ");
        }

        if out.is_empty() {
            out.push_str("None");
        }
        write!(f, "{}", out.trim())
    }
}
