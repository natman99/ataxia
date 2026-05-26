use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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
