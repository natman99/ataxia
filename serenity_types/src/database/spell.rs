use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spell {
    pub index: String,
    pub name: String,
    pub desc: Vec<String>,
    #[serde(default)]
    pub higher_level: Vec<String>,
    pub range: String,
    pub components: Vec<String>,
    pub material: Option<String>,
    pub ritual: bool,
    pub duration: String,
    pub concentration: bool,
    pub casting_time: String,
    pub level: i64,
    pub attack_type: Option<String>,
    pub damage: Option<Damage>,
    pub school: School,
    pub classes: Vec<Class>,
    pub subclasses: Vec<Subclass>,
    pub url: String,
    pub dc: Option<Dc>,
    pub heal_at_slot_level: Option<HealAtSlotLevel>,
    pub area_of_effect: Option<AreaOfEffect>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Damage {
    pub damage_type: Option<DamageType>,
    pub damage_at_slot_level: Option<DamageAtSlotLevel>,
    pub damage_at_character_level: Option<DamageAtCharacterLevel>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageType {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageAtSlotLevel {
    #[serde(rename = "3")]
    pub n3: Option<String>,
    #[serde(rename = "6")]
    pub n6: Option<String>,
    #[serde(rename = "7")]
    pub n7: Option<String>,
    #[serde(rename = "8")]
    pub n8: Option<String>,
    #[serde(rename = "9")]
    pub n9: Option<String>,
    #[serde(rename = "4")]
    pub n4: Option<String>,
    #[serde(rename = "5")]
    pub n5: Option<String>,
    #[serde(rename = "1")]
    pub n1: Option<String>,
    #[serde(rename = "2")]
    pub n2: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageAtCharacterLevel {
    #[serde(rename = "1")]
    pub n1: String,
    #[serde(rename = "5")]
    pub n5: String,
    #[serde(rename = "11")]
    pub n11: String,
    #[serde(rename = "17")]
    pub n17: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct School {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Class {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subclass {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dc {
    pub dc_type: DcType,
    pub dc_success: String,
    pub desc: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DcType {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealAtSlotLevel {
    #[serde(rename = "7")]
    pub n7: Option<String>,
    #[serde(rename = "2")]
    pub n2: Option<String>,
    #[serde(rename = "3")]
    pub n3: Option<String>,
    #[serde(rename = "4")]
    pub n4: Option<String>,
    #[serde(rename = "5")]
    pub n5: Option<String>,
    #[serde(rename = "6")]
    pub n6: Option<String>,
    #[serde(rename = "8")]
    pub n8: Option<String>,
    #[serde(rename = "9")]
    pub n9: Option<String>,
    #[serde(rename = "1")]
    pub n1: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AreaOfEffect {
    #[serde(rename = "type")]
    pub type_field: String,
    pub size: i64,
}
