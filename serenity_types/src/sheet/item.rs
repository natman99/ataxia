use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{roll::Die, sheet::roll::Roll};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
/// An item.
pub struct Item {
    /// Id of the item. This is an all lowercase version of the item's name with underscores instead of spaces.
    pub id: String,
    /// The name of the item
    pub name: String,
    /// The description of the item.
    pub description: String,
    /// How many of the item? Should never be zero.
    pub quantity: i32,
    /// The roll if the item deals damage or has an effect.
    pub roll: Option<Roll>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            name: "Example item".to_string(),
            description: "i explode something".to_string(),
            roll: Some(Roll::new(&[Die::D20], 0)),
            quantity: 1,
            id: "example_item".to_string(),
        }
    }
}
