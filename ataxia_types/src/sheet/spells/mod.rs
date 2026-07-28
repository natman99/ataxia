use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// use crate::database::spell::Spell;

mod spell;
pub use spell::Spell;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, JsonSchema)]
#[serde(default)]
/// A character's spells.
pub struct Spells {
    /// Map of spells
    pub spells: BTreeMap<String, Spell>,
}
