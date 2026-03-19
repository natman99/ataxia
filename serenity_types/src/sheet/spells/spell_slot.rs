use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, JsonSchema)]
/// The available spell slots.
pub struct SpellSlots {
    pub slots: Vec<SpellSlot>,
}

impl SpellSlots {
    /// Check if a spell slot is available.
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
/// A spell slot.
pub struct SpellSlot {
    /// The level of the spell slot.
    level: SpellLevel,
    /// Whether the slot is spent or not.
    pub spent: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize, Hash, JsonSchema)]
/// The level of the spell slot.
pub struct SpellLevel(pub u8);
