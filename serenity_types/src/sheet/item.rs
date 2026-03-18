use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{roll::Die, sheet::roll::Roll};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
/// An item.
pub struct Item {
    name: String,
    description: String,
    roll: Roll,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            name: "example item".to_string(),
            description: "i explode something".to_string(),
            roll: Roll::new(&[Die::D20], 0),
        }
    }
}
