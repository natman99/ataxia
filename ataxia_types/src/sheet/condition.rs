use std::fmt::Display;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::database;

#[derive(Debug, Clone, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
/// Conditions affecting the character.
pub struct Conditions {
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
/// A condition including a name and duration.
pub struct Condition {
    pub name: String,
    pub desc: String,
    pub duration: i32,
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            name: "Stunned".to_string(),
            desc: "".to_string(),
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
        Self {
            name,
            duration,
            desc: "".to_string(),
        }
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

impl From<database::condition::Condition> for Condition {
    fn from(value: database::condition::Condition) -> Self {
        Self {
            name: value.name,
            desc: value.desc.join("\n"),
            duration: -1,
        }
    }
}
