pub mod basic;
pub mod load_sheet;

#[derive(Debug, thiserror::Error)]
#[error("Init Error")]
pub struct InitError;
