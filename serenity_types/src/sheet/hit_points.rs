use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AbilityScores, Level,
    roll::{Die, Roll},
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
/// The hit points of the character. Default display displays the current health.
pub struct HitPoints {
    /// Current health.
    pub current: i32,
    /// Max health.
    pub max: u32,
    /// Bonus health gained per level.
    pub level_bonus: i32,
    /// Temporary bonus health.
    pub bonus: i32,
}

impl Default for HitPoints {
    fn default() -> Self {
        Self {
            current: 12,
            max: 12,
            level_bonus: Default::default(),
            bonus: Default::default(),
        }
    }
}

impl HitPoints {
    pub fn new(max: u32) -> Self {
        Self {
            current: max as i32,
            max: max,
            level_bonus: 0,
            bonus: 0,
        }
    }
    /// Calculate new max hp. Only modifies max hp.
    pub fn fixed(self, level: Level, hit_dice: Die, scores: &AbilityScores) -> Self {
        let die_max = hit_dice.max() as i32;
        let con_mod = scores.con.modifier();

        let fixed_per_level = (die_max / 2 + 1) as i32 + scores.con.modifier() + self.level_bonus;

        let mut total: i32 = die_max + con_mod + self.level_bonus;

        if level.0 >= 2 {
            total += fixed_per_level * (level.0 as i32 - 1);
        }
        let mut s = self;

        println!("{}", fixed_per_level);
        println!("{}", total);

        s.max = total.max(1) as u32;

        s
    }
    /// Apply damage. Damage is subtracted.
    pub fn hit(&mut self, dmg: i32) {
        if self.bonus > 0 {
            self.bonus = self.bonus - dmg;
            if self.bonus < 0 {
                self.current -= self.bonus.abs();
                self.bonus = 0;
            }
        } else {
            self.current -= dmg;
        }
        self.current = self.current.max(0);
    }
    pub fn heal(&mut self, healing: i32) {
        self.current += healing;
        self.current = self.current.max(self.max as i32);
    }
}

impl Display for HitPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.current)
    }
}
