use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::Display;

use crate::{AbilityScores, Score};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct Senses {
    pub perception: i32,
    pub investigation: i32,
    pub insight: i32,
    pub extra: ExtraSenses,
}

impl Senses {
    pub fn new(scores: &AbilityScores) -> Self {
        Self {
            perception: scores.get(&Score::Wis).modifier(),
            investigation: scores.get(&Score::Int).modifier(),
            insight: scores.get(&Score::Wis).modifier(),
            extra: Default::default(),
        }
    }
}

impl Default for Senses {
    fn default() -> Self {
        Self {
            perception: 10,
            investigation: 10,
            insight: 10,
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct ExtraSenses {
    pub blind_sight: bool,
    pub dark_vision: bool,
    pub tremor_sense: bool,
    pub true_sight: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Display)]
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
