use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hash, Default)]
/// Conditions affecting the character.
pub struct Conditions {
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Hash)]
/// A condition including a name and duration.
pub struct Condition {
    pub name: String,
    pub duration: i32,
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            name: "Stunned".to_string(),
            duration: -1,
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let turns = if self.duration < 0 {
            "inf".to_string()
        } else {
            format!("{} turns", self.duration)
        };
        write!(f, "{}: {}", self.name, turns)
    }
}

impl Condition {
    pub fn new(name: String, duration: i32) -> Self {
        Self { name, duration }
    }

    pub fn tick(&mut self) {
        if self.duration == 0 {
            return;
        }
        self.duration += -1;
    }

    pub fn expired(&self) -> bool {
        self.duration == 0
    }
}
