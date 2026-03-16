use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::roll::{Die, Roll};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Ability {
    name: String,
    description: Option<String>,
    roll: Option<Roll>,
}

impl Default for Ability {
    fn default() -> Self {
        Self {
            name: "example spell".to_string(),
            description: Some("i explode something".to_string()),
            roll: Some(Roll::new(&[Die::D20], 0)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct Abilities {
    pub inner: HashMap<String, Ability>,
}
