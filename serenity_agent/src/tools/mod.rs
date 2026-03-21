use std::sync::Arc;

use parking_lot::Mutex;
use serenity_types::Character;

pub mod basic;
pub mod health;
pub use health::{damage, heal};

pub mod ability_scores;
pub mod get_sheet;
pub mod save_sheet;
pub mod skills;

#[derive(Debug, thiserror::Error)]
#[error("Init Error")]
pub struct InitError;

pub type SheetState = Arc<Mutex<Character>>;
