use std::{
    fmt::Display,
    ops::{Deref, Index, IndexMut},
    str::FromStr,
};

use rand::RngExt;
#[cfg(feature = "rhai")]
use rhai::CustomType;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{AbilityScores, HitPoints, roll::Die};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
pub struct Classes {
    pub classes: Vec<Class>,
}

impl Classes {
    pub fn heal(
        &mut self,
        index: usize,
        roll: bool,
        health: &mut HitPoints,
        ability_scores: &AbilityScores,
    ) {
        self[index].heal(roll, health, ability_scores);
    }

    pub fn heal_first(
        &mut self,
        roll: bool,
        health: &mut HitPoints,
        ability_scores: &AbilityScores,
    ) -> Option<()> {
        if let Some(class) = self
            .classes
            .iter_mut()
            .find(|f| f.healing_die_remaining > 0)
        {
            class.heal(roll, health, ability_scores);
            Some(())
        } else {
            None
        }
    }

    #[cfg(feature = "rhai")]
    pub fn get(&mut self, index: i64) -> Class {
        self.classes[index as usize].clone()
    }

    #[cfg(feature = "rhai")]
    pub fn set(&mut self, index: i64, value: Class) {
        self.classes[index as usize] = value;
    }
}

impl Default for Classes {
    fn default() -> Self {
        let class = Class::default();
        Self {
            classes: vec![class],
        }
    }
}

impl Display for Classes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in &self.classes {
            s.push_str(&format!("{} {} ", i.class, i.level));
        }
        write!(f, "{}", s)
    }
}

impl Deref for Classes {
    type Target = Vec<Class>;

    fn deref(&self) -> &Self::Target {
        &self.classes
    }
}

impl Index<usize> for Classes {
    type Output = Class;

    fn index(&self, index: usize) -> &Self::Output {
        &self.classes[index]
    }
}

impl IndexMut<usize> for Classes {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.classes[index]
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "rhai", derive(CustomType))]
pub struct Class {
    pub class: ClassType,
    pub subclass: String,
    #[cfg_attr(feature = "rhai", rhai_type(get = Self::get_level), rhai_type(set = Self::set_level))]
    pub level: Level,
    pub hit_dice: Die,
    pub health_bonus_per_level: i64,
    pub healing_die_remaining: i64,
    pub max_healing_die: i64,
}

impl Default for Class {
    fn default() -> Self {
        let class: ClassType = Default::default();
        let hit_dice = class.get_hit_dice();
        Self {
            class,
            level: Default::default(),
            hit_dice,
            health_bonus_per_level: 0,
            healing_die_remaining: 1,
            subclass: String::new(),
            max_healing_die: 1,
        }
    }
}

impl Class {
    pub fn heal(&mut self, roll: bool, health: &mut HitPoints, ability_scores: &AbilityScores) {
        if self.healing_die_remaining == 0 {
            return;
        }
        if roll {
            let mut rng = rand::rng();
            let healing_amount =
                rng.random_range(1..=self.hit_dice.max()) as i32 + ability_scores.con.modifier();
            health.heal(healing_amount);
        }
        self.healing_die_remaining = self.healing_die_remaining.saturating_sub(1);
    }

    pub fn get_level(&self) -> i64 {
        self.level.0
    }

    pub fn set_level(&mut self, level: i64) -> () {
        self.level.0 = level;
        self.max_healing_die = level;
    }

    pub fn set_class(&mut self, class: &str) -> () {
        // Cannot fail
        let Ok(class) = ClassType::from_str(class) else {
            return;
        };
        self.class = class;
        self.hit_dice = self.class.get_hit_dice();
    }

    pub fn new(level: Level, class: &str) -> Self {
        let class = ClassType::from_str(class).expect("Cannot fail");
        Self {
            subclass: "".to_string(),
            level,
            hit_dice: class.get_hit_dice(),
            health_bonus_per_level: 0,
            healing_die_remaining: level.0 as i64,
            class,
            max_healing_die: level.0 as i64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(from = "String"))]
/// The class of a character. `from_str` is infailable on this type.
/// Implements `From<&str>` and `From<String>`
pub enum ClassType {
    #[default]
    Fighter,
    Wizard,
    Sorcerer,
    Cleric,
    Barbarian,
    Monk,
    Ranger,
    Paladin,
    Rogue,
    Warlock,
    Bard,
    Druid,
    Artificer,
    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

impl From<String> for ClassType {
    fn from(value: String) -> Self {
        Self::from_str(&value).unwrap()
    }
}

impl From<&str> for ClassType {
    fn from(value: &str) -> Self {
        Self::from_str(value).unwrap()
    }
}

impl FromStr for ClassType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "fighter" => ClassType::Fighter,
            "wizard" => ClassType::Wizard,
            "sorcerer" => ClassType::Sorcerer,
            "cleric" => ClassType::Cleric,
            "barbarian" => ClassType::Barbarian,
            "monk" => ClassType::Monk,
            "ranger" => ClassType::Ranger,
            "paladin" => ClassType::Paladin,
            "rogue" => ClassType::Rogue,
            "warlock" => ClassType::Warlock,
            "bard" => ClassType::Bard,
            "druid" => ClassType::Druid,
            _ => ClassType::Other(s.to_string()),
        })
    }
}

impl ClassType {
    pub fn get_hit_dice(&self) -> Die {
        match self {
            ClassType::Fighter => Die::D10,
            ClassType::Wizard => Die::D6,
            ClassType::Sorcerer => Die::D6,
            ClassType::Cleric => Die::D8,
            ClassType::Barbarian => Die::D12,
            ClassType::Monk => Die::D8,
            ClassType::Ranger => Die::D10,
            ClassType::Paladin => Die::D10,
            ClassType::Rogue => Die::D8,
            ClassType::Warlock => Die::D8,
            ClassType::Bard => Die::D8,
            ClassType::Druid => Die::D8,
            ClassType::Artificer => Die::D8,
            ClassType::Other(_) => Die::D8,
        }
    }
}

impl Display for ClassType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = match &self {
            ClassType::Fighter => "Fighter",
            ClassType::Wizard => "Wizard",
            ClassType::Sorcerer => "Sorcerer",
            ClassType::Cleric => "Cleric",
            ClassType::Barbarian => "Barbarian",
            ClassType::Monk => "Monk",
            ClassType::Ranger => "Ranger",
            ClassType::Paladin => "Paladin",
            ClassType::Rogue => "Rogue",
            ClassType::Warlock => "Warlock",
            ClassType::Bard => "Bard",
            ClassType::Druid => "Druid",
            ClassType::Artificer => "Artificer",
            ClassType::Other(name) => name.as_str(),
        };

        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Level(pub i64);

impl Default for Level {
    fn default() -> Self {
        Self(1)
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::class::ClassType;

    #[test]
    fn from_other() {
        ClassType::from_str("test").unwrap();
    }
}
