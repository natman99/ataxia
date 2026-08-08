use std::io::IsTerminal;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "rhai")]
use rhai::CustomType;

use crate::{Score, sheet::AbilityScores, skills::Skills};
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
pub struct SavingThrows {
    pub str: bool,
    pub dex: bool,
    pub con: bool,
    pub wis: bool,
    pub int: bool,
    pub cha: bool,
}

impl SavingThrows {
    /// Get the saving throw
    /// bonus.
    pub fn check(&self, score: &Score, ability_scores: &AbilityScores, skills: &Skills) -> i64 {
        let base = ability_scores.get(score).modifier();

        let has_bonus = match score {
            Score::Str => self.str,
            Score::Dex => self.dex,
            Score::Con => self.con,
            Score::Int => self.int,
            Score::Wis => self.wis,
            Score::Cha => self.cha,
        };
        let bonus = if has_bonus {
            skills.proficiency_bonus
        } else {
            0
        };

        base + bonus
    }

    pub fn proficient(&self, score: &Score) -> bool {
        match score {
            Score::Str => self.str,
            Score::Dex => self.dex,
            Score::Con => self.con,
            Score::Int => self.int,
            Score::Wis => self.wis,
            Score::Cha => self.cha,
        }
    }
}
