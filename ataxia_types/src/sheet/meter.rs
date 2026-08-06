use std::collections::HashMap;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "rhai")]
use rhai::CustomType;

use crate::sheet::source::{HasSource, Source};

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
pub struct Meters {
    pub meters: HashMap<String, Meter>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
/// A meter item. These are for counting any restorable consumable resources including spell slots.
pub struct Meter {
    pub name: String,
    pub slot_number: u32,
    pub spent: u32,

    pub restore: RestoreTime,
    pub source: Option<Source>,
}

impl Meter {
    pub fn new(name: String, slot_number: u32, restore: RestoreTime) -> Self {
        Self {
            name,
            restore,
            slot_number,
            spent: 0,
            source: None,
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

impl HasSource for Meter {
    fn source(&self) -> Option<&Source> {
        self.source.as_ref()
    }

    fn add_source(&mut self, source: Source) {
        self.source = Some(source)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
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
            short_rest: false,
            long_rest: true,
        }
    }
}
