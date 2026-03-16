mod spell_slot;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
pub use spell_slot::*;

use crate::database::spell::Spell;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Spells {
    spell_slots: SpellSlots,
    spells: SpellList,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SpellList {
    inner: BTreeMap<String, Spell>,
}
