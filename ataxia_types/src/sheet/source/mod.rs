use std::fmt::Display;

#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub trait HasSource {
    fn source(&self) -> Option<&Source>;
    fn add_source(&mut self, source: Source);
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum Source {
    Single(String),
    Book { book: String, page: u32 },
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Single(s) => write!(f, "{s}"),
            Source::Book { book, page } => write!(f, "{} (p. {})", book, page),
        }
    }
}
