use std::fmt::Display;

use enumflags2::{BitFlag, BitFlags, bitflags};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::sheet::AbilityScores;

#[bitflags]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
/// Skills enum.
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl Skill {
    pub const ALL_STR: [&'static str; 18] = [
        "acrobatics",
        "animalHandling",
        "arcana",
        "athletics",
        "deception",
        "history",
        "insight",
        "intimidation",
        "investigation",
        "medicine",
        "nature",
        "perception",
        "performance",
        "persuasion",
        "religion",
        "sleightOfHand",
        "stealth",
        "survival",
    ];

    pub const ALL_STR_PRETTY: [&'static str; 18] = [
        "Acrobatics",
        "Animal Handling",
        "Arcana",
        "Athletics",
        "Deception",
        "History",
        "Insight",
        "Intimidation",
        "Investigation",
        "Medicine",
        "Nature",
        "Perception",
        "Performance",
        "Persuasion",
        "Religion",
        "Sleight Of Hand",
        "Stealth",
        "Survival",
    ];

    pub const ALL_ITER: [Skill; 18] = [
        Skill::Acrobatics,
        Skill::AnimalHandling,
        Skill::Arcana,
        Skill::Athletics,
        Skill::Deception,
        Skill::History,
        Skill::Insight,
        Skill::Intimidation,
        Skill::Investigation,
        Skill::Medicine,
        Skill::Nature,
        Skill::Perception,
        Skill::Performance,
        Skill::Persuasion,
        Skill::Religion,
        Skill::SleightOfHand,
        Skill::Stealth,
        Skill::Survival,
    ];
}

impl Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Skill::Athletics => "Athletics",
            Skill::Acrobatics => "Acrobatics",
            Skill::SleightOfHand => "Sleight Of Hand",
            Skill::Stealth => "Stealth",
            Skill::Arcana => "Arcana",
            Skill::History => "History",
            Skill::Investigation => "Investigation",
            Skill::Nature => "Nature",
            Skill::Religion => "Religion",
            Skill::AnimalHandling => "Animal Handling",
            Skill::Insight => "Insight",
            Skill::Medicine => "Medicine",
            Skill::Perception => "Perception",
            Skill::Survival => "Survival",
            Skill::Deception => "Deception",
            Skill::Intimidation => "Intimidation",
            Skill::Performance => "Performance",
            Skill::Persuasion => "Persuasion",
        };

        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for Skill {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "athletics" => Ok(Skill::Athletics),
            "acrobatics" => Ok(Skill::Acrobatics),
            "sleightOfHand" => Ok(Skill::SleightOfHand),
            "stealth" => Ok(Skill::Stealth),
            "arcana" => Ok(Skill::Arcana),
            "history" => Ok(Skill::History),
            "investigation" => Ok(Skill::Investigation),
            "nature" => Ok(Skill::Nature),
            "religion" => Ok(Skill::Religion),
            "animalHandling" => Ok(Skill::AnimalHandling),
            "insight" => Ok(Skill::Insight),
            "medicine" => Ok(Skill::Medicine),
            "perception" => Ok(Skill::Perception),
            "survival" => Ok(Skill::Survival),
            "deception" => Ok(Skill::Deception),
            "intimidation" => Ok(Skill::Intimidation),
            "performance" => Ok(Skill::Performance),
            "persuasion" => Ok(Skill::Persuasion),
            _ => Err(()),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
#[serde(default)]
struct SkillsJson {
    athletics: bool,
    acrobatics: bool,
    sleightofhand: bool,
    stealth: bool,
    arcana: bool,
    history: bool,
    investigation: bool,
    nature: bool,
    religion: bool,
    animalhandling: bool,
    insight: bool,
    medicine: bool,
    perception: bool,
    survival: bool,
    deception: bool,
    intimidation: bool,
    performance: bool,
    persuasion: bool,
    proficiency_bonus: u32,
    expertise: Vec<Skill>,
}

#[derive(Debug, Clone, PartialEq, Default, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
#[serde(from = "SkillsJson")]
#[serde(into = "SkillsJson")]
/// The characters proficiency skills.
pub struct Skills {
    pub proficiency_bonus: u32,
    pub proficiencies: BitFlags<Skill>,
    pub expertise: BitFlags<Skill>,
}

impl From<SkillsJson> for Skills {
    fn from(value: SkillsJson) -> Self {
        let mut out = Skills::default();
        out.proficiencies.set(Skill::Athletics, value.athletics);
        out.proficiencies.set(Skill::Acrobatics, value.acrobatics);
        out.proficiencies
            .set(Skill::SleightOfHand, value.sleightofhand);
        out.proficiencies.set(Skill::Stealth, value.stealth);
        out.proficiencies.set(Skill::Arcana, value.arcana);
        out.proficiencies.set(Skill::History, value.history);
        out.proficiencies
            .set(Skill::Investigation, value.investigation);
        out.proficiencies.set(Skill::Nature, value.nature);
        out.proficiencies.set(Skill::Religion, value.religion);
        out.proficiencies
            .set(Skill::AnimalHandling, value.animalhandling);
        out.proficiencies.set(Skill::Insight, value.insight);
        out.proficiencies.set(Skill::Medicine, value.medicine);
        out.proficiencies.set(Skill::Perception, value.perception);
        out.proficiencies.set(Skill::Survival, value.survival);
        out.proficiencies.set(Skill::Deception, value.deception);
        out.proficiencies
            .set(Skill::Intimidation, value.intimidation);
        out.proficiencies.set(Skill::Performance, value.performance);
        out.proficiencies.set(Skill::Persuasion, value.persuasion);
        out.proficiency_bonus = value.proficiency_bonus;
        out.expertise = {
            let mut m: BitFlags<Skill> = BitFlags::empty();
            value.expertise.iter().for_each(|f| m.set(*f, true));
            m
        };
        out
    }
}

impl From<&Skills> for SkillsJson {
    fn from(value: &Skills) -> Self {
        let mut out = SkillsJson::default();
        out.proficiency_bonus = value.proficiency_bonus;
        value.proficiencies.iter().for_each(|f| match f {
            Skill::Athletics => out.athletics = true,
            Skill::Acrobatics => out.acrobatics = true,
            Skill::SleightOfHand => out.sleightofhand = true,
            Skill::Stealth => out.stealth = true,
            Skill::Arcana => out.arcana = true,
            Skill::History => out.history = true,
            Skill::Investigation => out.investigation = true,
            Skill::Nature => out.nature = true,
            Skill::Religion => out.religion = true,
            Skill::AnimalHandling => out.animalhandling = true,
            Skill::Insight => out.insight = true,
            Skill::Medicine => out.medicine = true,
            Skill::Perception => out.perception = true,
            Skill::Survival => out.survival = true,
            Skill::Deception => out.deception = true,
            Skill::Intimidation => out.intimidation = true,
            Skill::Performance => out.performance = true,
            Skill::Persuasion => out.persuasion = true,
        });

        out.expertise = {
            let mut m = vec![];
            value.expertise.into_iter().for_each(|f| m.push(f));
            m
        };

        out
    }
}
impl From<Skills> for SkillsJson {
    fn from(value: Skills) -> Self {
        Self::from(&value)
    }
}
impl Skills {
    /// Get the skill bonus.
    pub fn check(&self, skill: &Skill, ability_scores: &AbilityScores) -> i32 {
        let base_modifier = match skill {
            //str scaling
            Skill::Athletics => ability_scores.str.modifier(),
            // dex scaling
            Skill::Acrobatics | Skill::SleightOfHand | Skill::Stealth => {
                ability_scores.dex.modifier()
            }
            // int scaling
            Skill::Arcana
            | Skill::History
            | Skill::Investigation
            | Skill::Nature
            | Skill::Religion => ability_scores.int.modifier(),
            // wis scaling
            Skill::AnimalHandling
            | Skill::Insight
            | Skill::Medicine
            | Skill::Perception
            | Skill::Survival => ability_scores.wis.modifier(),
            // char scaling
            Skill::Deception | Skill::Intimidation | Skill::Performance | Skill::Persuasion => {
                ability_scores.char.modifier()
            }
        };

        let bonus = if self.proficiencies.contains(*skill) {
            self.proficiency_bonus
        } else {
            0
        };

        let bonus = if self.expertise.contains(*skill) {
            bonus * 2
        } else {
            bonus
        };

        let total = base_modifier + bonus as i32;
        total
    }
}
