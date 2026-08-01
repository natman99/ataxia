use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;

pub type Traits = Vec<Trait>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trait {
    pub index: String,
    pub races: Vec<Race>,
    pub subraces: Vec<Subrace>,
    pub name: String,
    pub desc: Vec<String>,
    pub proficiencies: Vec<Proficiency>,
    #[serde(rename = "proficiency_choices")]
    pub proficiency_choices: Option<ProficiencyChoices>,
    #[serde(rename = "trait_specific")]
    pub trait_specific: Option<TraitSpecific>,
    #[serde(rename = "language_options")]
    pub language_options: Option<LanguageOptions>,
    pub parent: Option<Parent>,
}

impl Display for Trait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}\n Races: {}",
            self.name,
            self.desc.join("\n"),
            self.subraces
                .iter()
                .map(|f| f.name.as_str())
                .collect::<Vec<&str>>()
                .join(", ")
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    pub index: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subrace {
    pub index: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proficiency {
    pub index: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProficiencyChoices {
    pub choose: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub from: From,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct From {
    #[serde(rename = "option_set_type")]
    pub option_set_type: String,
    pub options: Vec<TraitOption>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraitOption {
    #[serde(rename = "option_type")]
    pub option_type: String,
    pub item: Item,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub index: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraitSpecific {
    #[serde(rename = "damage_type")]
    pub damage_type: Option<DamageType>,
    #[serde(rename = "breath_weapon")]
    pub breath_weapon: Option<BreathWeapon>,
    #[serde(rename = "subtrait_options")]
    pub subtrait_options: Option<SubtraitOptions>,
    #[serde(rename = "spell_options")]
    pub spell_options: Option<SpellOptions>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageType {
    pub index: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreathWeapon {
    pub name: String,
    pub desc: String,
    #[serde(rename = "area_of_effect")]
    pub area_of_effect: AreaOfEffect,
    pub usage: Usage,
    pub dc: Dc,
    pub damage: Vec<Damage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaOfEffect {
    pub size: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "type")]
    pub type_field: String,
    pub times: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dc {
    #[serde(rename = "dc_type")]
    pub dc_type: DcType,
    #[serde(rename = "success_type")]
    pub success_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DcType {
    pub index: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    #[serde(rename = "damage_type")]
    pub damage_type: DamageType2,
    #[serde(rename = "damage_at_character_level")]
    pub damage_at_character_level: DamageAtCharacterLevel,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageType2 {
    pub index: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageAtCharacterLevel {
    #[serde(rename = "1")]
    pub n1: String,
    #[serde(rename = "6")]
    pub n6: String,
    #[serde(rename = "11")]
    pub n11: String,
    #[serde(rename = "16")]
    pub n16: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtraitOptions {
    pub choose: i64,
    pub from: From2,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct From2 {
    #[serde(rename = "option_set_type")]
    pub option_set_type: String,
    pub options: Vec<Option2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Option2 {
    #[serde(rename = "option_type")]
    pub option_type: String,
    pub item: Item2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item2 {
    pub index: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpellOptions {
    pub choose: i64,
    pub from: From3,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct From3 {
    #[serde(rename = "option_set_type")]
    pub option_set_type: String,
    pub options: Vec<Option3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Option3 {
    #[serde(rename = "option_type")]
    pub option_type: String,
    pub item: Item3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item3 {
    pub index: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageOptions {
    pub choose: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub from: From4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct From4 {
    #[serde(rename = "option_set_type")]
    pub option_set_type: String,
    pub options: Vec<Option4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Option4 {
    #[serde(rename = "option_type")]
    pub option_type: String,
    pub item: Item4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item4 {
    pub index: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    pub index: String,
    pub name: String,
}
