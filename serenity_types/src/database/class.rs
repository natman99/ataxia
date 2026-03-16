use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub index: String,
    pub name: String,
    #[serde(rename = "hit_die")]
    pub hit_die: i64,
    #[serde(rename = "proficiency_choices")]
    pub proficiency_choices: Vec<ProficiencyChoice>,
    pub proficiencies: Vec<Proficiency>,
    #[serde(rename = "saving_throws")]
    pub saving_throws: Vec<SavingThrow>,
    #[serde(rename = "starting_equipment")]
    pub starting_equipment: Vec<StartingEquipment>,
    #[serde(rename = "starting_equipment_options")]
    pub starting_equipment_options: Vec<StartingEquipmentOption>,
    #[serde(rename = "class_levels")]
    pub class_levels: String,
    #[serde(rename = "multi_classing")]
    pub multi_classing: MultiClassing,
    pub subclasses: Vec<Subclass>,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProficiencyChoice {
    pub desc: String,
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
    pub options: Vec<ClassOption>,
}

pub enum OptionSetType {
    OptionsArray(Vec<ClassOption>),
    EquipmentCategory(Item),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(rename = "option")]
pub enum ClassOption {
    Reference(Item),
    CountedReference { count: i32, of: Item },
}

impl Default for ClassOption {
    fn default() -> Self {
        Self::Reference(Default::default())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proficiency {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavingThrow {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartingEquipment {
    pub equipment: Equipment,
    pub quantity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Equipment {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartingEquipmentOption {
    pub desc: String,
    pub choose: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub from: From2,
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
    pub count: Option<i64>,
    pub of: Option<Of>,
    pub choice: Option<Choice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Of {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub desc: String,
    pub choose: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub from: From3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct From3 {
    #[serde(rename = "option_set_type")]
    pub option_set_type: String,
    #[serde(rename = "equipment_category")]
    pub equipment_category: EquipmentCategory,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipmentCategory {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiClassing {
    pub prerequisites: Vec<Prerequisite>,
    pub proficiencies: Vec<Proficiency2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prerequisite {
    #[serde(rename = "ability_score")]
    pub ability_score: AbilityScore,
    #[serde(rename = "minimum_score")]
    pub minimum_score: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityScore {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proficiency2 {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subclass {
    pub index: String,
    pub name: String,
    pub url: String,
}
