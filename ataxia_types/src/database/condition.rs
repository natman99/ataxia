use std::fmt::Display;

#[cfg(feature = "schema")]
use schemars::JsonSchema;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", serde(from = "SkillsJson"))]
#[cfg_attr(feature = "serde", serde(into = "SkillsJson"))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
struct Temp {}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Condition {
    /// Machine index
    pub index: String,
    /// Human readable name
    pub name: String,
    /// Effect description
    pub desc: Vec<String>,
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&self.name);
        s.push('\n');
        self.desc
            .iter()
            .for_each(|f| s.push_str(&format!("{}\n", f)));

        write!(f, "{}", s)
    }
}
