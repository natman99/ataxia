use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::database::spell::Spell;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, JsonSchema)]
/// A character's spells.
pub struct Spells {
    /// Map of spells
    pub spells: BTreeMap<String, Spell>,
}
