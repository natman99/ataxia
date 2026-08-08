#[rhai::export_module]
pub mod library {
    use ataxia_types::{
        Initiative, Item,
        class::Class,
        feat::Feat,
        feature::{Effect, Feature, HasFeature},
        meter::{Meter, Meters, RestoreTime},
        roll::{Die, Roll},
        saving_throws::SavingThrows,
        sheet::HitPoints,
        source::{HasSource, Source},
    };
    use rhai::{Array, Dynamic, ImmutableString};

    // -- Top-level functions --

    pub fn class() -> Class {
        Class::default()
    }

    pub fn standard_array() -> Array {
        [8, 10, 12, 13, 14, 15]
            .iter()
            .map(|f| Dynamic::from_int(*f))
            .collect::<Vec<Dynamic>>()
            .into()
    }

    pub fn set_bonus(initiative: &mut Initiative, b: i64) {
        initiative.bonus = b;
    }

    pub fn set_max(hp: &mut HitPoints, v: i64) {
        hp.max = v;
    }

    pub fn set_current(hp: &mut HitPoints, v: i64) {
        hp.current = v;
    }

    pub fn set_level_bonus(hp: &mut HitPoints, v: i64) {
        hp.level_bonus = v;
    }

    pub fn set_subclass(class: &mut Class, new: ImmutableString) {
        class.subclass = new.to_string();
    }

    // -- meter submodule --

    pub mod meter {
        use super::*;

        pub fn meter(name: ImmutableString, slots: i64) -> Meter {
            Meter::new(name.to_string(), slots as u32, RestoreTime::default())
        }

        pub fn short_rest(m: &mut Meter) {
            m.restore.short_rest = true;
        }

        pub fn long_rest(m: &mut Meter) {
            m.restore.long_rest = true;
        }

        pub fn add(meters: &mut Meters, m: Meter) {
            meters.meters.insert(m.name.to_string(), m);
        }

        pub fn source(m: &mut Meter, source: ImmutableString) {
            m.add_source(Source::Single(source.to_string()));
        }
    }

    // -- class submodule --

    pub mod class {
        use std::str::FromStr;

        use super::*;

        pub fn set_class(class: &mut Class, s: ImmutableString) {
            class.set_class(s.as_str());
        }

        pub fn set_level(class: &mut Class, level: i64) {
            class.set_level(level);
        }

        pub fn set_hit_die(class: &mut Class, die: ImmutableString) {
            let Ok(d) = Die::from_str(die.as_str()) else {
                return;
            };
            class.hit_dice = d;
        }
    }

    // -- spell submodule --

    pub mod spell {
        use std::str::FromStr;

        use super::*;
        use ataxia_types::{
            Score,
            damage::DamageType,
            spells::{self, Area, Component, Damage, Heal, Spell, SpellType, Spells, Success},
        };

        pub fn add(spells: &mut Spells, s: Spell) {
            spells.spells.insert(s.name.clone(), s);
        }

        pub fn source(f: &mut Spell, source: ImmutableString) {
            f.add_source(Source::Single(source.to_string()));
        }

        pub fn component(spell: &mut Spell, f: ImmutableString) {
            let Ok(c) = Component::from_str(f.as_str()) else {
                return;
            };
            spell.components.push(c);
        }

        pub fn area(spell: &mut Spell, shape: ImmutableString, size: i64) {
            spell.area = Some(Area {
                shape: shape.to_string(),
                size: size as i32,
            });
        }

        pub fn desc(spell: &mut Spell, desc: ImmutableString) {
            spell.desc = desc.to_string();
        }

        pub fn description(spell: &mut Spell, desc: ImmutableString) {
            spell.desc = desc.to_string();
        }

        pub fn cast_time(spell: &mut Spell, cast_time: ImmutableString) {
            spell.cast_time = cast_time.to_string();
        }

        pub fn range(spell: &mut Spell, range: ImmutableString) {
            spell.range = range.to_string();
        }

        pub fn duration(spell: &mut Spell, duration: ImmutableString) {
            spell.duration = duration.to_string();
        }

        pub fn concentration(spell: &mut Spell, concentration: bool) {
            spell.concentration = concentration;
        }

        pub fn ritual(spell: &mut Spell, ritual: bool) {
            spell.ritual = ritual;
        }

        pub fn level(spell: &mut Spell, level: i64) {
            spell.level = level;
        }

        pub fn automatic(spell: &mut Spell) {
            spell.spell_type = SpellType::Automatic;
        }

        pub fn melee(spell: &mut Spell) {
            spell.spell_type = SpellType::Melee;
        }

        pub fn ranged(spell: &mut Spell) {
            spell.spell_type = SpellType::Ranged;
        }

        pub fn saving(spell: &mut Spell, dc_type: ImmutableString) {
            let Ok(s) = Score::from_str(dc_type.as_str()) else {
                return;
            };
            spell.spell_type = SpellType::Saving {
                dc_type: s,
                success: ataxia_types::spells::Success::Half,
            };
        }

        pub fn saving_success(
            spell: &mut Spell,
            dc_type: ImmutableString,
            success: ImmutableString,
        ) {
            let Ok(s) = Score::from_str(dc_type.as_str()) else {
                return;
            };
            let Ok(success) = Success::from_str(success.as_str()) else {
                return;
            };
            spell.spell_type = SpellType::Saving {
                dc_type: s,
                success,
            };
        }

        pub fn damage(spell: &mut Spell, damage: ImmutableString, element: ImmutableString) {
            let Ok(r) = Roll::from_str(damage.as_str()) else {
                return;
            };

            let Ok(element) = DamageType::from_str(element.as_str()) else {
                return;
            };
            spell.effect = ataxia_types::spells::Effect::Damage(Damage {
                damage: r,
                damage_type: element,
            });
        }

        pub fn name(s: &mut Spell, n: ImmutableString) {
            s.name = n.to_string();
        }

        pub fn spell(name: ImmutableString) -> Spell {
            Spell {
                name: name.to_string(),
                ..Default::default()
            }
        }

        pub fn element(spell: &mut Spell, element: ImmutableString) {
            if let spells::Effect::Damage(d) = &mut spell.effect {
                let Ok(element) = DamageType::from_str(element.as_str()) else {
                    return;
                };
                d.damage_type = element
            }
        }

        pub fn heal(spell: &mut Spell, healing: ImmutableString) {
            let heal = match Roll::from_str(healing.as_str()) {
                Ok(r) => Heal::Roll(r),
                Err(_) => Heal::Static(healing.to_string()),
            };
            spell.effect = spells::Effect::Heal(heal);
        }

        pub fn school(spell: &mut Spell, school: ImmutableString) {
            let Ok(s) = spells::School::from_str(school.as_str()) else {
                return;
            };
            spell.school = s;
        }
    }

    // -- item submodule --

    pub mod item {
        use super::*;

        #[rhai_fn(name = "item")]
        pub fn item_name_desc(name: ImmutableString, description: ImmutableString) -> Item {
            Item {
                name: name.to_string(),
                description: description.to_string(),
                ..Default::default()
            }
        }

        #[rhai_fn(name = "item")]
        pub fn item_name_roll(name: ImmutableString, roll: Roll) -> Item {
            Item {
                name: name.to_string(),
                roll: Some(roll),
                ..Default::default()
            }
        }

        #[rhai_fn(name = "item")]
        pub fn item_name_desc_roll(
            name: ImmutableString,
            description: ImmutableString,
            roll: Roll,
        ) -> Item {
            Item {
                name: name.to_string(),
                description: description.to_string(),
                roll: Some(roll),
                ..Default::default()
            }
        }

        pub fn add(inventory: &mut ataxia_types::Inventory, item: Item) {
            inventory.0.insert(item.name.to_string(), item);
        }

        pub fn desc(f: &mut Item, desc: ImmutableString) {
            f.description = desc.to_string()
        }

        pub fn description(f: &mut Item, desc: ImmutableString) {
            f.description = desc.to_string()
        }

        pub fn count(f: &mut Item, count: i64) {
            f.quantity = count as i32;
        }

        pub fn quantity(f: &mut Item, count: i64) {
            f.quantity = count as i32;
        }

        pub fn feature(f: &mut Item, feature: Effect) {
            f.add(feature);
        }

        pub fn source(f: &mut Item, source: ImmutableString) {
            f.add_source(Source::Single(source.to_string()));
        }
    }

    // -- feat submodule --

    pub mod feat {
        use super::*;

        pub fn source(f: &mut Feat, source: ImmutableString) {
            f.add_source(Source::Single(source.to_string()));
        }

        pub fn feature(f: &mut Feat, feature: Effect) {
            f.add(feature);
        }

        pub fn source_feature(f: &mut Feature, source: ImmutableString) {
            f.add_source(Source::Single(source.to_string()));
        }
    }

    // -- roll submodule --

    pub mod roll {
        use std::str::FromStr;

        use super::*;

        pub fn roll(s: ImmutableString) -> Dynamic {
            if let Ok(r) = Roll::from_str(s.as_str()) {
                Dynamic::from(r)
            } else {
                Dynamic::UNIT
            }
        }
    }

    // -- saving_throws submodule --

    pub mod saving_throws {
        use super::*;

        #[rhai_fn(get = "str")]
        pub fn str_get(st: &mut SavingThrows) -> bool {
            st.str
        }

        #[rhai_fn(set = "str")]
        pub fn str_set(st: &mut SavingThrows, v: bool) {
            st.str = v;
        }

        #[rhai_fn(get = "dex")]
        pub fn dex_get(st: &mut SavingThrows) -> bool {
            st.dex
        }

        #[rhai_fn(set = "dex")]
        pub fn dex_set(st: &mut SavingThrows, v: bool) {
            st.dex = v;
        }

        #[rhai_fn(get = "con")]
        pub fn con_get(st: &mut SavingThrows) -> bool {
            st.con
        }

        #[rhai_fn(set = "con")]
        pub fn con_set(st: &mut SavingThrows, v: bool) {
            st.con = v;
        }

        #[rhai_fn(get = "int")]
        pub fn int_get(st: &mut SavingThrows) -> bool {
            st.int
        }

        #[rhai_fn(set = "int")]
        pub fn int_set(st: &mut SavingThrows, v: bool) {
            st.int = v;
        }

        #[rhai_fn(get = "wis")]
        pub fn wis_get(st: &mut SavingThrows) -> bool {
            st.wis
        }

        #[rhai_fn(set = "wis")]
        pub fn wis_set(st: &mut SavingThrows, v: bool) {
            st.wis = v;
        }

        #[rhai_fn(get = "cha")]
        pub fn cha_get(st: &mut SavingThrows) -> bool {
            st.cha
        }

        #[rhai_fn(set = "cha")]
        pub fn cha_set(st: &mut SavingThrows, v: bool) {
            st.cha = v;
        }
    }

    // -- skills submodule --

    pub mod skills {
        use super::*;
        use ataxia_types::skills::Skill;

        pub fn add(skills: &mut ataxia_types::skills::Skills, name: ImmutableString) {
            let Ok(skill) = Skill::try_from(name.as_str()) else {
                return;
            };
            skills.proficiencies.set(skill, true);
        }

        pub fn expertise(skills: &mut ataxia_types::skills::Skills, name: ImmutableString) {
            let Ok(skill) = Skill::try_from(name.as_str()) else {
                return;
            };
            skills.proficiencies.set(skill, true);
            skills.expertise.set(skill, true);
        }
    }
}
