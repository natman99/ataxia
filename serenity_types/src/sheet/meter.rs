use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, JsonSchema)]
pub struct Meters {
    pub meters: HashMap<String, Meter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, JsonSchema)]
#[serde(default)]
/// A meter item. These are for counting any consumable resources including spell slots.
pub struct Meter {
    pub slot_number: u32,
    pub spent: u32,

    pub restore: RestoreTime,
}

impl Meter {
    pub fn new(name: String, slot_number: u32, restore: RestoreTime) -> Self {
        Self {
            restore,
            slot_number,
            spent: 0,
        }
    }

    /// Check if a meter slot is available.
    pub fn check(&self) -> bool {
        self.spent < self.slot_number
    }
    pub fn spend(&mut self) {
        if self.spent < self.slot_number {
            self.spent += 1;
        }
    }

    pub fn restore(&mut self) {
        self.spent = self.spent.saturating_sub(1);
    }
    /// Restore all spell slots.
    pub fn restore_all(&mut self) -> () {
        self.spent = 0;
    }

    pub fn add(&mut self) {
        self.slot_number += 1;
    }

    pub fn remove(&mut self) {
        self.slot_number = self.slot_number.saturating_sub(1);
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(default)]
/// When the slot is restored. These can both be false.
pub struct RestoreTime {
    /// Restore on short rest.
    pub short_rest: bool,
    /// Restore on long rest.
    pub long_rest: bool,
}

impl Default for RestoreTime {
    fn default() -> Self {
        Self {
            short_rest: Default::default(),
            long_rest: true,
        }
    }
}
