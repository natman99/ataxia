use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, JsonSchema)]
pub struct Meters {
    pub meters: HashMap<String, Meter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, JsonSchema)]
/// A meter item. These are for counting any consumable resources including spell slots.
pub struct Meter {
    /// The name of the meter.
    pub name: String,

    pub slots: Vec<Metric>,
    pub restore: RestoreTime,
}

impl Meter {
    pub fn new(
        name: String,
        slot_number: usize,
        level: Option<SpellLevel>,
        restore: RestoreTime,
    ) -> Self {
        let slot = Metric {
            level: level.unwrap_or(SpellLevel::default()),
            spent: false,
        };

        let slots = vec![slot; slot_number];
        Self {
            name,
            slots: slots,
            restore,
        }
    }

    /// Check if a meter slot is available.
    pub fn check(&self, level: SpellLevel) -> bool {
        self.slots
            .iter()
            .any(|f| f.spent == false && f.level == level)
    }
    /// Returns true if the slot was successfully spent.
    pub fn spend(&mut self, level: SpellLevel) -> bool {
        if self.check(level) {
            let idx = self
                .slots
                .iter()
                .position(|f| f.spent == true && f.level == level)
                .expect("We checked this before");
            self.slots[idx].spent = false;

            true
        } else {
            false
        }
    }

    /// Returns true if the slot was successfully restored.
    pub fn restore(&mut self, level: SpellLevel) -> bool {
        if !self.check(level) {
            let idx = self
                .slots
                .iter()
                .position(|f| f.spent == false && f.level == level)
                .expect("We checked this before");
            self.slots[idx].spent = true;

            true
        } else {
            false
        }
    }
    /// Restore all spell slots.
    pub fn restore_all(&mut self) -> () {
        self.slots.iter_mut().for_each(|f| f.spent = false);
    }

    pub fn add(&mut self, s: Metric) {
        self.slots.push(s);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
/// A metric slot.
pub struct Metric {
    /// The level of the spell slot.
    pub level: SpellLevel,
    /// Whether the slot is spent or not.
    pub spent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize, Hash, JsonSchema, Default)]
/// The level of the spell slot.
pub struct SpellLevel(pub u8);

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash, Serialize, Deserialize, JsonSchema, Default,
)]
/// When the slot is restored. These can both be false.
pub struct RestoreTime {
    /// Restore on short rest.
    pub short_rest: bool,
    /// Restore on long rest.
    pub long_rest: bool,
}
