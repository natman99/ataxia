use std::fmt::Display;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature= "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, PartialEq)]
/// Generic struct for name and description for general types.
/// Most types in the DB have name & desc.
pub struct Generic {
    pub name: String,
    pub desc: StringOrVec,
}
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, PartialEq)]
#[serde(untagged)]
pub enum StringOrVec {
    Single(String),
    Multiple(Vec<String>),
}

impl Display for Generic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n\n{}", self.name, self.desc)
    }
}

impl Display for StringOrVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringOrVec::Single(s) => write!(f, "{s}"),
            StringOrVec::Multiple(items) => write!(f, "{}", items.join("\n")),
        }
    }
}
