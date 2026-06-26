use core::str;
use std::{collections::HashMap, fmt::Display, sync::LazyLock};

use rand::{Rng, RngExt};
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub enum RollModifier {
    Advantage,
    Disadvantage,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
/// A dice roll. Can contain multiple dice.
pub struct Roll {
    /// Array of dice to roll.
    pub dice: Vec<Die>,
    /// A bonus to add after the dice have been rolled.
    pub bonus: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RollResult {
    /// The total dice results added up.
    pub total: i32,
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
            bonus: bonus,
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
            total,
            num_dice_rolled: results.len() as i32,
            results,
            failures,
        }
    }

    // pub fn max(&self) -> u32 {}
}

impl TryFrom<&str> for Roll {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        static M: LazyLock<Regex> = LazyLock::new(|| regex::Regex::new(r"(\d+)[dD](\d+)").unwrap());

        if let Some(matches) = M.captures(value) {
            let n = matches[0].parse::<u32>();
            let d = matches[1].parse::<u32>();

            if let Ok(n) = n
                && let Ok(d) = d
                && let Ok(die) = Die::try_from(d)
            {
                let bonus = {
                    if value.contains("+") {
                        if let Some(e) = value.split("+").collect::<Vec<&str>>().get(1) {
                            e.parse().unwrap_or(0)
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                };
                Ok(Self {
                    dice: vec![die; n as usize],
                    bonus: bonus,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl TryFrom<String> for Roll {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Roll::try_from(value.as_str())
    }
}

impl Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        let mut m = HashMap::new();
        for i in &self.dice {
            if m.contains_key(&i.max()) {
                let c = m.get_mut(&i.max()).unwrap();
                *c += 1;
            } else {
                m.insert(i.max(), 1);
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
/// An individual die.
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    #[default]
    D20,
    Percentile,
}

impl TryFrom<u32> for Die {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            4 => Ok(Self::D4),
            6 => Ok(Self::D6),
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
