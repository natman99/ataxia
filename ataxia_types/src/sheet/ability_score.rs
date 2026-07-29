use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
/// An ability score. Values default to ten.
pub struct AbilityScore {
    /// Base score.
    base: i32,
    /// Bonus (from class or otherwise).
    bonus: i32,
}

impl Default for AbilityScore {
    fn default() -> Self {
        Self { base: 10, bonus: 0 }
    }
}

impl AbilityScore {
    pub fn new(base: i32, bonus: i32) -> Self {
        Self {
            base: base.max(1),
            bonus,
        }
    }
    /// Get the total score.
    pub fn get(&self) -> i32 {
        self.base + self.bonus
    }

    pub fn get_bonus(&self) -> i32 {
        self.bonus
    }

    pub fn set_bonus(&mut self, num: i32) {
        self.bonus = num;
    }

    pub fn add_bonus(&mut self, num: i32) {
        self.bonus += num;
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
