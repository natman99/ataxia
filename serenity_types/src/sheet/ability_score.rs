use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
/// An ability score. Values default to ten.
pub struct AbilityScore {
    /// Base score.
    base: u32,
    /// Bonus (from class or otherwise).
    bonus: u32,
    /// Flat override of both other values.
    override_score: Option<u32>,
}

impl Default for AbilityScore {
    fn default() -> Self {
        Self {
            base: 10,
            bonus: 0,
            override_score: None,
        }
    }
}

impl AbilityScore {
    pub fn new(base: u32, bonus: u32) -> Self {
        Self {
            base,
            bonus,
            override_score: None,
        }
    }
    /// Get the total score.
    pub fn get(&self) -> u32 {
        self.override_score.unwrap_or(self.base + self.bonus)
    }

    pub fn get_bonus(&self) -> u32 {
        self.bonus
    }

    pub fn set_bonus(&mut self, num: u32) {
        self.bonus = num;
    }

    /// Remove all bonuses.
    pub fn reset(&mut self) {
        self.bonus = 0;
    }

    pub fn modifier(&self) -> i32 {
        match self.base + self.bonus {
            1 => -5,
            2..=3 => -4,
            4..=5 => -3,
            6..=7 => -2,
            8..=9 => -1,
            10..=11 => 0,
            12..=13 => 1,
            14..=15 => 2,
            16..=17 => 3,
            18..=19 => 4,
            20..=21 => 5,
            22..=23 => 6,
            24..=25 => 7,
            26..=27 => 8,
            28..=29 => 9,
            30 => 10,
            _ => {
                eprintln!("Invalid ability score, returning 0");
                0
            }
        }
    }
}
