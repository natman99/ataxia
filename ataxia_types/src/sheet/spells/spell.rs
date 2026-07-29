use std::str::FromStr;

use anyhow::{Context, anyhow};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::{
    Character, Score,
    class::Level,
    damage::DamageType,
    database::spell::DatabaseSpell,
    roll::{Roll, Rollable},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Spell {
    pub name: String,
    pub desc: Vec<String>,
    pub range: String,
    pub cast_time: String,
    pub concentration: bool,
    pub duration: String,
    pub ritual: bool,
    pub level: i32,
    pub spell_type: SpellType,
    pub effect: Effect,
    pub school: School,
    pub area: Option<Area>,
    pub components: Vec<Component>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Area {
    shape: String,
    size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema, Display)]
pub enum Effect {
    Damage(Damage),
    Heal(Heal),
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema, Display)]
pub enum Heal {
    Static(String),
    Roll(Roll),
}

#[derive(
    EnumString,
    Display,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    JsonSchema,
    Serialize,
    Deserialize,
)]
pub enum Component {
    #[strum(serialize = "V")]
    Verbal,
    #[strum(serialize = "S")]
    Semantic,
    #[strum(serialize = "M")]
    Material,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct Damage {
    pub damage_type: DamageType,
    pub damage: Roll,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema, EnumString)]
pub enum SpellType {
    /// Saving throw
    Saving { dc_type: Score, success: Success },
    /// Ranged attack roll
    Ranged,
    /// Melee attack roll
    Melee,
    /// Automatic success
    Automatic,
}

/// Effect on DC success
#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema, EnumString, Display, Default,
)]
#[strum(ascii_case_insensitive)]
pub enum Success {
    #[default]
    Half,
    None,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum School {
    Conjuration,
    Necromancy,
    Evocation,
    Abjuration,
    Transmutation,
    Divination,
    Enchantment,
    Illusion,
    Chronurgy,
}

impl Rollable for Spell {
    fn roll(
        &self,
        rng: &mut impl rand::Rng,
        special: Option<crate::roll::RollModifier>,
    ) -> Option<crate::roll::RollResult> {
        Some(match &self.effect {
            Effect::Damage(damage) => damage.damage.roll(rng, special),
            Effect::Heal(heal) => match heal {
                Heal::Static(_) => return None,
                Heal::Roll(roll) => roll.roll(rng, special),
            },
            Effect::Other => return None,
        })
    }
}

impl Spell {
    fn try_from_database(value: DatabaseSpell, sheet: &Character) -> anyhow::Result<Self> {
        let mut components = vec![];
        for i in value.components {
            let a = Component::from_str(&i).context("Missing component")?;
            components.push(a);
        }

        let effect = {
            if let Some(heal) = value.heal_at_slot_level {
                let heal = heal.n1.or_else(|| {
                    heal.n2.or_else(|| {
                        heal.n3.or_else(|| {
                            heal.n4.or_else(|| {
                                heal.n5.or_else(|| {
                                    heal.n6
                                        .or_else(|| heal.n7.or_else(|| heal.n8.or_else(|| heal.n9)))
                                })
                            })
                        })
                    })
                });
                let Some(heal) = heal else {
                    return Err(anyhow!("Healing missing"));
                };

                let modifier = sheet.ability_modifier;
                let modifier = sheet.ability_scores.get(&modifier).get();
                let heal = heal.replace("MOD", &modifier.to_string());
                let heal = if let Ok(roll) = Roll::from_str(&heal) {
                    Heal::Roll(roll)
                } else {
                    Heal::Static(heal)
                };
                Effect::Heal(heal)
            } else if let Some(damage) = value.damage {
                let damage_type: DamageType = if let Some(t) = damage.damage_type
                    && let Ok(d) = DamageType::from_str(&t.index)
                {
                    d
                } else {
                    DamageType::None
                };

                let roll = if let Some(a) = damage.damage_at_character_level {
                    let d = match sheet.class[0].level.0 {
                        1..5 => a.n1,
                        5..11 => a.n5,
                        11..14 => a.n11,
                        17..20 => a.n17,
                        _ => unreachable!(),
                    };
                    let r = Roll::from_str(&d).context("Failed to parse character damage roll")?;
                    Some(r)
                } else if let Some(a) = damage.damage_at_slot_level {
                    let d = a.n1.or_else(|| {
                        a.n2.or_else(|| {
                            a.n3.or_else(|| {
                                a.n4.or_else(|| {
                                    a.n5.or_else(|| {
                                        a.n6.or_else(|| a.n7.or_else(|| a.n8.or_else(|| a.n9)))
                                    })
                                })
                            })
                        })
                    });
                    let Some(d) = d else {
                        return Err(anyhow!("Damage not found"));
                    };
                    Some(Roll::from_str(&d).context("Failed to parse slot damage roll")?)
                } else {
                    None
                };
                let Some(roll) = roll else {
                    return Err(anyhow!("Roll not found"));
                };
                let damage = Damage {
                    damage_type,
                    damage: roll,
                };
                Effect::Damage(damage)
            } else {
                Effect::Other
            }
        };

        let spell_type = {
            if let Some(dc) = value.dc {
                let dc_type =
                    Score::from_str(&dc.dc_type.index).context("Failed to parse score")?;
                let success =
                    Success::from_str(&dc.dc_success).context("Failed to parse success")?;
                SpellType::Saving { dc_type, success }
            } else if let Some(attack_type) = value.attack_type
                && let Ok(attack) = SpellType::from_str(&attack_type)
            {
                attack
            } else {
                SpellType::Automatic
            }
        };

        let s = Self {
            name: value.name,
            desc: value.desc,
            range: value.range,
            cast_time: value.casting_time,
            concentration: value.concentration,
            duration: value.duration,
            ritual: value.ritual,
            level: value.level as i32,
            spell_type,
            effect,
            school: School::from_str(&value.school.name).context("Failed to parse school")?,
            area: if let Some(a) = value.area_of_effect {
                Some(Area {
                    shape: a.type_field,
                    size: a.size as i32,
                })
            } else {
                None
            },
            components,
        };

        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, str::FromStr};

    use super::*;

    #[test]
    fn test_school() {
        School::from_str("Conjuration").unwrap();
        let s = School::from_str("conjuration").unwrap();
        assert_eq!(s.to_string(), "Conjuration");
    }
    #[test]
    fn test_spell_from() {
        let path = "../5e-database/src/2014/en/5e-SRD-Spells.json";

        let s = fs::read_to_string(path).unwrap();
        let spells: Vec<DatabaseSpell> = serde_json::from_str(&s).unwrap();
        let sheet = Character::default();
        for i in spells {
            let name = i.name.clone();
            Spell::try_from_database(i, &sheet)
                .context(format!("{}", name))
                .unwrap();
        }
    }
}
