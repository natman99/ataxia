/// Structs to read 5e-database from dnd srd api.
// pub mod class;
pub mod condition;

#[cfg(feature = "serde")]
pub mod generic;

#[cfg(feature = "serde")]
pub mod race;

#[cfg(feature = "serde")]
pub mod spell;
#[cfg(feature = "serde")]
pub mod traits;
