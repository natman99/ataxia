use std::fmt::Display;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "rhai")]
use rhai::CustomType;

use strum::{Display, EnumString};

use crate::{AbilityScores, Score};

#[derive(Debug, Clone, PartialEq, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
#[derive(Default)]
pub struct Senses {
    pub perception_bonus: i64,
    pub investigation_bonus: i64,
    pub insight_bonus: i64,
    pub extra: ExtraSenses,
}

impl Senses {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_perception(&self, scores: &AbilityScores) -> i64 {
        scores.get(&Score::Wis).modifier() + self.perception_bonus
    }

    pub fn get_insight(&self, scores: &AbilityScores) -> i64 {
        scores.get(&Score::Wis).modifier() + self.insight_bonus
    }

    pub fn get_investigation(&self, scores: &AbilityScores) -> i64 {
        scores.get(&Score::Int).modifier() + self.investigation_bonus
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
pub struct ExtraSenses {
    pub blind_sight: bool,
    pub dark_vision: bool,
    pub tremor_sense: bool,
    pub true_sight: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[strum(ascii_case_insensitive)]
/// Extra sense types.
pub enum Sense {
    #[strum(serialize = "blind_sight")]
    BlindSight,
    #[strum(serialize = "dark_vision")]
    DarkVision,
    #[strum(serialize = "tremor_sense")]
    TremorSense,
    #[strum(serialize = "true_sight")]
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
