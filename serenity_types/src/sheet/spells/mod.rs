mod spell_slot;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use spell_slot::*;
use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::database::spell::Spell;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, JsonSchema)]
/// A character's spells.
pub struct Spells {
    spell_slots: SpellSlots,
    spells: SpellList,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, JsonSchema)]
// #[serde(from = "HashMap<String, Spell>")]
// #[serde(into = "HashMap<String, Spell>")]
/// Map of spells by name.
pub struct SpellList {
    inner: BTreeMap<String, Spell>,
}
