#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// use crate::database::spell::Spell;

mod spell;
pub use spell::Spell;

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
/// A character's spells.
pub struct Spells {
    /// Map of spells
    pub spells: BTreeMap<String, Spell>,
}
