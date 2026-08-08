use std::fmt::Display;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "rhai")]
use rhai::CustomType;

use crate::{AbilityScores, class::Classes};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
/// The hit points of the character. Default display displays the current health.
pub struct HitPoints {
    /// Current health. Must never be higher than the max.
    pub current: i64,
    /// Max health.
    pub max: i64,
    /// Bonus health gained per level.
    pub level_bonus: i64,
    /// Temporary bonus health.
    pub bonus: i64,
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
            current: max as i64,
            max: max as i64,
            level_bonus: 0,
            bonus: 0,
        }
    }
    /// Calculate new max hp. Only modifies max hp.
    pub fn fixed(self, scores: &AbilityScores, classes: &Classes) -> Self {
        let base_class = &classes[0];

        let base = classes[0].hit_dice.max();
        let die_max = base;
        let con_mod = scores.con.modifier();

        let mut total: i64 =
            base as i64 + con_mod + self.level_bonus + base_class.health_bonus_per_level as i64;

        let fixed_per_level = (die_max / 2 + 1) as i64
            + scores.con.modifier() as i64
            + self.level_bonus
            + base_class.health_bonus_per_level as i64;

        if base_class.level.0 >= 2 {
            total += fixed_per_level * (base_class.level.0 as i64 - 1);
        }

        for i in &classes.classes[1..] {
            let die_max = i.hit_dice.max();

            let fixed_per_level = (die_max / 2 + 1) as i64
                + scores.con.modifier()
                + self.level_bonus
                + i.health_bonus_per_level;

            for _ in 0..i.level.0 {
                total += fixed_per_level
            }
        }

        let mut s = self;

        s.max = total.max(1);

        s
    }
    /// Apply damage. Damage is subtracted.
    pub fn hit(&mut self, dmg: i64) {
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
    pub fn heal(&mut self, healing: i64) {
        self.current += healing;
        self.current = self.current.min(self.max);
    }
}

impl Display for HitPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.current)
    }
}
