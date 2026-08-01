#[cfg(feature= "serde")]
use serde::Deserialize;
#[cfg(feature= "serde")]
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Race {
    pub index: String,
    pub name: String,
    pub speed: i64,
    pub ability_bonuses: Vec<AbilityBonuse>,
    pub alignment: String,
    pub age: String,
    pub size: String,
    pub size_description: String,
    pub languages: Vec<Language>,
    pub language_desc: String,
    pub traits: Vec<Trait>,
    pub subraces: Vec<Subrace>,
    pub url: String,
    pub language_options: Option<LanguageOptions>,
    pub ability_bonus_options: Option<AbilityBonusOptions>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbilityBonuse {
    pub ability_score: AbilityScore,
    pub bonus: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbilityScore {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Language {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trait {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subrace {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageOptions {
    pub choose: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub from: From,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct From {
    pub option_set_type: String,
    pub options: Vec<RaceOption>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "option")]
pub struct RaceOption {
    pub option_type: String,
    pub item: Item,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbilityBonusOptions {
    pub choose: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub from: From2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct From2 {
    pub option_set_type: String,
    pub options: Vec<Option2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Option2 {
    pub option_type: String,
    pub ability_score: AbilityScore2,
    pub bonus: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbilityScore2 {
    pub index: String,
    pub name: String,
    pub url: String,
}
