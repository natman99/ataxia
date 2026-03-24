use enumflags2::BitFlags;
use enumflags2::bitflags;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Senses {
    pub perception: i32,
    pub investigation: i32,
    pub insight: i32,
    pub extra: ExtraSenses,
}

impl Default for Senses {
    fn default() -> Self {
        Self {
            perception: 10,
            investigation: 10,
            insight: 10,
            extra: Default::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
pub struct ExtraSenses {
    pub blind_sight: bool,
    pub dark_vision: bool,
    pub tremor_sense: bool,
    pub true_sight: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
/// Extra sense types.
pub enum Sense {
    BlindSight,
    DarkVision,
    TremorSense,
    TrueSight,
}

// impl Display for ExtraSenses {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let s = match self {
//             ExtraSenses::BlindSight => "Blindsight",
//             ExtraSenses::DarkVision => "Darkvision",
//             ExtraSenses::TremorSense => "Tremorsense",
//             ExtraSenses::TrueSight => "Truesight",
//         };

//         write!(f, "{}", s)
//     }
// }
