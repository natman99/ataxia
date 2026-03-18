use std::collections::{BTreeMap, HashMap};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::roll::{Die, Roll};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct Feature {
    name: String,
    description: Option<String>,
    roll: Option<Roll>,
}

impl Default for Feature {
    fn default() -> Self {
        Self {
            name: "example spell".to_string(),
            description: Some("i explode something".to_string()),
            roll: Some(Roll::new(&[Die::D20], 0)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, JsonSchema)]
pub struct Features {
    pub inner: BTreeMap<String, Feature>,
}
