use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    roll::{Die, Rollable},
    sheet::roll::Roll,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
/// An item.
pub struct Item {
    /// The name of the item
    pub name: String,
    /// The description of the item.
    pub description: String,
    /// How many of the item.
    pub quantity: i32,
    /// The roll if the item deals damage or has an effect.
    pub roll: Option<Roll>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            name: "Example item".to_string(),
            description: "Example description".to_string(),
            roll: None,
            quantity: 1,
        }
    }
}

impl Item {
    pub fn with_roll(mut self, roll: Roll) -> Self {
        self.roll = Some(roll);
        self
    }

    pub fn with_desc(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_quantity(mut self, quantity: i32) -> Self {
        self.quantity = quantity;
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

impl Rollable for Item {
    fn roll(
        &self,
        rng: &mut impl rand::Rng,
        special: Option<super::roll::RollModifier>,
    ) -> Option<super::roll::RollResult> {
        if let Some(ref r) = self.roll {
            Some(r.roll(rng, special))
        } else {
            None
        }
    }
}
