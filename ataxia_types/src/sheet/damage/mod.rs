use std::str::FromStr;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Serialize,
    Deserialize,
    EnumString,
    JsonSchema,
)]
#[strum(ascii_case_insensitive)]
pub enum DamageType {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
    None,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_damage() {
        DamageType::from_str("acid").unwrap();

        DamageType::from_str("Acid").unwrap();
    }
}
