use core::str;
use std::sync::LazyLock;

use rand::{Rng, RngExt};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Roll {
    pub dice: Vec<Die>,
    pub bonus: i32,
}

pub struct RollResult {
    pub total: i32,
    pub num_dice_rolled: i32,
    pub results: Vec<i32>,
}

impl Roll {
    pub fn new(dice: &[Die], bonus: i32) -> Self {
        Self {
            dice: Vec::from(dice),
            bonus: bonus,
        }
    }

    pub fn roll<T: Rng>(&self, rng: &mut T) -> RollResult {
        let mut total = 0i32;
        let mut results = vec![];

        for i in &self.dice {
            let r = i.roll(rng);
            total += r;
            results.push(r);
        }

        total += self.bonus;

        RollResult {
            total,
            num_dice_rolled: results.len() as i32,
            results,
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
                Ok(Self {
                    dice: vec![die; n as usize],
                    bonus: 0,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
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
