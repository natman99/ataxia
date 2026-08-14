use core::str;
use std::{collections::HashMap, fmt::Display, str::FromStr, sync::LazyLock};

use rand::{Rng, RngExt};
use regex::Regex;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "rhai")]
use rhai::CustomType;
use strum::EnumString;

/// Trait for dice rolls.
pub trait Rollable {
    /// Returns the roll result if the type has a valid roll on it.
    fn roll(&self, rng: &mut impl rand::Rng, special: Option<RollModifier>) -> Option<RollResult>;
}

pub enum RollModifier {
    Advantage,
    Disadvantage,
}

#[derive(thiserror::Error, Debug)]
pub enum RollParseError {
    #[error("Parsing failed")]
    ParseFailed,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default), serde(try_from = "String"))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
/// A dice roll. Can contain multiple dice.
pub struct Roll {
    /// Array of dice to roll.
    pub dice: Vec<Die>,
    /// A bonus to add after the dice have been rolled.
    pub bonus: i32,
}

impl Default for Roll {
    fn default() -> Self {
        Self {
            dice: vec![Die::D20],
            bonus: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RollResult {
    /// The total dice results added up.
    pub total: i64,
    /// The number of dice rolled.
    pub num_dice_rolled: i32,
    /// The list of individual results.
    pub results: Vec<i32>,
    /// The dice that weren't counted due to advantage/disadvantage
    pub failures: Vec<i32>,
}

impl Roll {
    pub fn new(dice: &[Die], bonus: i32) -> Self {
        Self {
            dice: Vec::from(dice),
            bonus,
        }
    }

    pub fn roll<T: Rng>(&self, rng: &mut T, special: Option<RollModifier>) -> RollResult {
        let mut total = 0i32;
        let mut results = vec![];
        let mut failures = vec![];

        for i in &self.dice {
            let r = i.roll(rng);

            if let Some(s) = &special {
                let other = i.roll(rng);

                let (success, failure) = {
                    match s {
                        RollModifier::Advantage => {
                            if other > r {
                                (other, r)
                            } else {
                                (r, other)
                            }
                        }
                        RollModifier::Disadvantage => {
                            if other < r {
                                (other, r)
                            } else {
                                (r, other)
                            }
                        }
                    }
                };

                total += success;
                results.push(success);
                failures.push(failure);
            } else {
                total += r;
                results.push(r);
            }
        }

        total += self.bonus;

        RollResult {
            total: total.into(),
            num_dice_rolled: results.len() as i32,
            results,
            failures,
        }
    }

    // pub fn max(&self) -> u32 {}
}

impl FromStr for Roll {
    type Err = RollParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        static M: LazyLock<Regex> = LazyLock::new(|| regex::Regex::new(r"(\d+)[dD](\d+)").unwrap());

        let mut dice = vec![];
        for m in M.captures_iter(value) {
            let n = m[1].parse::<u32>();
            let d = m[2].parse::<u32>();

            if let Ok(n) = n
                && let Ok(d) = d
                && let Ok(die) = Die::try_from(d)
            {
                dice.append(&mut vec![die; n as usize]);
            } else {
                continue;
            }
        }

        let bonus = {
            let mut b = 0i32;
            for i in value.split("+") {
                if let Ok(e) = i.trim().parse::<i32>() {
                    b += e;
                }
            }
            b
        };

        if dice.is_empty() && bonus == 0 {
            return Err(RollParseError::ParseFailed);
        }
        Ok(Self { dice, bonus })
    }
}

impl TryFrom<String> for Roll {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::from_str(value.as_str()).map_err(|_| {
            "Failed to parse roll. 'mod' and other placeholders are not supported".to_string()
        })?)
    }
}

impl Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        let mut m = HashMap::new();
        for i in &self.dice {
            if let std::collections::hash_map::Entry::Vacant(e) = m.entry(i.max()) {
                e.insert(1);
            } else {
                let c = m.get_mut(&i.max()).unwrap();
                *c += 1;
            }
        }

        for i in m.iter() {
            out.push_str(&format!("{}d{} ", i.1, i.0));
        }
        if self.bonus != 0 {
            out.push_str(&format!("+ {}", self.bonus));
        }

        write!(f, "{}", out)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, EnumString)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[strum(ascii_case_insensitive)]
/// An individual die.
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    #[default]
    D20,
    #[strum(serialize = "D100")]
    Percentile,
}

impl TryFrom<u32> for Die {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            4 => Ok(Self::D4),
            6 => Ok(Self::D6),
            8 => Ok(Self::D8),
            10 => Ok(Self::D10),
            12 => Ok(Self::D12),
            20 => Ok(Self::D20),
            100 => Ok(Self::Percentile),
            _ => Err(()),
        }
    }
}

impl Die {
    pub fn max(&self) -> u32 {
        match self {
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
            Die::Percentile => 10,
        }
    }

    pub fn roll<T: Rng>(&self, rng: &mut T) -> i32 {
        let max = self.max();
        rng.random_range(1..=max) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_display() {
        let roll = Roll::from_str("2d6").unwrap();
        assert_eq!(roll.to_string().trim(), "2d6");
    }

    #[test]
    fn test_display_bonus() {
        let roll = Roll::from_str("2d6 + 2").unwrap();
        assert_eq!(roll.to_string().trim(), "2d6 + 2");
    }

    #[test]
    fn test_parse() {
        Roll::from_str("10d6").unwrap();
        Roll::from_str("1d6 + 10").unwrap();
        Roll::from_str("2d8").unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_die() {
        Roll::from_str("1d5").unwrap();
    }

    #[test]
    #[should_panic]
    fn empty() {
        Roll::from_str("").unwrap();
    }

    #[test]
    fn only_bonus() {
        Roll::from_str("20").unwrap();
    }
}
