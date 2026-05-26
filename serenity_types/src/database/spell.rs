use std::fmt::{Display, write};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Spell {
    /// Internal identifier for spell.
    pub index: String,
    /// Name to display.
    pub name: String,
    /// Description.
    pub desc: Vec<String>,
    #[serde(default)]
    /// How the spell is modified casting with a higher level spell slot.
    pub higher_level: Vec<String>,
    pub range: String,
    /// Required components for a spell. E.g. verbal, sematic, material.
    pub components: Vec<String>,
    /// Materials required for the spell.
    pub material: Option<String>,
    /// Whether the spell can be ritual cast or not.
    pub ritual: bool,
    /// Duration of the spell
    pub duration: String,
    pub concentration: bool,
    pub casting_time: String,
    pub level: i64,
    pub attack_type: Option<String>,
    pub damage: Option<Damage>,
    pub school: School,
    /// The classes that can use the spell,
    pub classes: Vec<Class>,
    /// The subclasses that can use the spell.
    pub subclasses: Vec<Subclass>,
    /// Relative api url of the spell. Ignore this.
    pub url: String,
    /// The dice check to hit if the spell has one.
    pub dc: Option<Dc>,
    pub heal_at_slot_level: Option<HealAtSlotLevel>,
    pub area_of_effect: Option<AreaOfEffect>,
}

impl Display for Spell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();

        out.push_str(&format!("{}\n", self.name));
        out.push_str(&format!(
            "Range: {}; Duration: {}; ",
            self.range, self.duration
        ));
        out.push_str(&format!(
            "Cast time: {}. Level: {};\n",
            self.casting_time, self.level
        ));

        if let Some(ref dmg) = self.damage {
            let mut inner = String::new();
            if let Some(ref d) = dmg.damage_at_slot_level {
                let t = "".to_string();
                let min_damage = {
                    d.n1.as_ref()
                        .unwrap_or(d.n2.as_ref().unwrap_or(d.n3.as_ref().unwrap_or(
                            d.n4.as_ref().unwrap_or(d.n5.as_ref().unwrap_or(
                                d.n6.as_ref().unwrap_or(d.n7.as_ref().unwrap_or(
                                    d.n8.as_ref().unwrap_or(d.n9.as_ref().unwrap_or(&t)),
                                )),
                            )),
                        )))
                };
                inner.push_str(&min_damage);
                inner.push(' ');

                if let Some(ref dmg_type) = dmg.damage_type {
                    inner.push_str(&dmg_type.name);

                    inner.push(' ');
                }

                if let Some(ref cantrip_dmg) = dmg.damage_at_character_level {
                    inner.push_str(&format!("{}", cantrip_dmg.n1));
                }

                inner.push('\n');
            }

            out.push_str(&inner);
        }
        {
            if self.concentration {
                out.push_str("Concentation; ");
            }

            if let Some(ref dc) = self.dc {
                out.push_str(&format!(
                    "DC: {}, Success: {}; ",
                    dc.dc_type.name, dc.dc_success
                ));
            }

            out.push_str(&format!("{}; ", self.components.join(",")));

            if self.components.is_empty() {
                out.push_str("None; ");
            }

            if self.ritual {
                out.push_str("Ritual;");
            }

            out.push_str(&format!("{};\n", self.school.name));
        }

        if let Some(ref m) = self.material {
            out.push_str(&format!("Requires: {}\n\n", m));
        }

        let mut desc = String::new();
        self.desc
            .iter()
            .for_each(|f| desc.push_str(&format!("{}\n", f)));

        out.push_str(&desc);
        out.push('\n');

        out.push_str(&format!(
            "Classes: {:#?}",
            self.classes
                .iter()
                .map(|f| f.name.as_str())
                .collect::<Vec<&str>>()
        ));

        write!(f, "{out}")
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Damage {
    pub damage_type: Option<DamageType>,
    pub damage_at_slot_level: Option<DamageAtSlotLevel>,
    pub damage_at_character_level: Option<DamageAtCharacterLevel>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DamageType {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct School {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Class {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Subclass {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Dc {
    pub dc_type: DcType,
    pub dc_success: String,
    pub desc: Option<String>,
}

impl Display for Dc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.dc_type.name, self.dc_success)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DcType {
    pub index: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AreaOfEffect {
    #[serde(rename = "type")]
    pub type_field: String,
    pub size: i64,
}
