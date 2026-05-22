use std::sync::Arc;

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serenity_types::Character;

pub mod basic;
pub mod health;
pub use health::{damage, heal};

pub mod ability_scores;
pub mod armor;
pub mod get_sheet;
pub mod inventory;
pub mod recalculate;

pub mod skills;

pub mod search;

pub mod feature;

#[derive(Debug, thiserror::Error)]
#[error("Init Error")]
pub struct InitError;

pub type SheetState = Arc<Mutex<Character>>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Success;

impl Success {
    fn success() -> String {
        "Success".to_string()
    }
}
