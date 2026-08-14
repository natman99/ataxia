use std::{any::TypeId, str::FromStr};

use ataxia_types::{
    Item,
    feat::Feat,
    feature::{Effect, HasFeature},
    senses::Sense,
    skills::Skill,
    source::HasSource,
};
use rhai::{Dynamic, ImmutableString};

#[rhai::export_module]
pub mod library {
    use std::str::FromStr;

    use ataxia_types::{
        Initiative, Item, Score,
        class::Class,
        feat::Feat,
        feature::{Effect, Feature, HasFeature},
        meter::{Meter, Meters, RestoreTime},
        roll::{Die, Roll},
        saving_throws::SavingThrows,
        senses::Senses,
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
    }

    pub fn set_bonus(initiative: &mut Initiative, b: i64) {
        initiative.bonus = b;
    }

    // ability modifier
    pub fn set_score(score: &mut Score, s: Score) {
        *score = s;
    }

    #[allow(non_upper_case_globals)]
    pub mod score {
        use ataxia_types::Score;

        pub const Str: Score = Score::Str;
        pub const Dex: Score = Score::Dex;
        pub const Con: Score = Score::Con;
        pub const Int: Score = Score::Int;
        pub const Wis: Score = Score::Wis;
        pub const Cha: Score = Score::Cha;
    }

    // -- health submodule --
    pub mod health {

        pub fn set_max(hp: &mut HitPoints, v: i64) {
            hp.max = v;
        }

        pub fn set_current(hp: &mut HitPoints, v: i64) {
            hp.current = v;
        }

        pub fn set_level_bonus(hp: &mut HitPoints, v: i64) {
            hp.level_bonus = v;
        }
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

        pub fn set_subclass(class: &mut Class, new: ImmutableString) {
            class.subclass = new.to_string();
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

        pub fn saving(spell: &mut Spell, dc_type: Score) {
            spell.spell_type = SpellType::Saving {
                dc_type,
                success: ataxia_types::spells::Success::Half,
            };
        }

        pub fn saving_success(spell: &mut Spell, dc_type: Score, success: ImmutableString) {
            let Ok(success) = Success::from_str(success.as_str()) else {
                return;
            };
            spell.spell_type = SpellType::Saving { dc_type, success };
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
            f.description = desc.to_string();
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

        pub fn roll(f: &mut Item, r: ImmutableString) {
            if let Ok(r) = Roll::from_str(r.as_str()) {
                f.roll = Some(r)
            }
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

        use ataxia_types::roll::Roll;

        use super::*;

        pub fn roll(s: ImmutableString) -> Dynamic {
            if let Ok(r) = Roll::from_str(s.as_str()) {
                Dynamic::from(r)
            } else {
                Dynamic::UNIT
            }
        }

        #[rhai_fn(name = "roll")]
        pub fn roll_roll(r: &mut Roll) -> i64 {
            let mut rng = rand::rng();
            r.roll(&mut rng, None).total
        }
    }

    // -- senses submodule --

    pub mod senses {
        use super::*;

        #[rhai_fn(get = "darkvision", pure)]
        pub fn darkvision_get(s: &mut Senses) -> bool {
            s.extra.dark_vision
        }

        #[rhai_fn(set = "darkvision")]
        pub fn darkvision_set(s: &mut Senses, v: bool) {
            s.extra.dark_vision = v;
        }

        #[rhai_fn(get = "tremor_sense", pure)]
        pub fn tremor_sense_get(s: &mut Senses) -> bool {
            s.extra.tremor_sense
        }

        #[rhai_fn(set = "tremor_sense")]
        pub fn tremor_sense_set(s: &mut Senses, v: bool) {
            s.extra.tremor_sense = v;
        }

        #[rhai_fn(get = "blindsight", pure)]
        pub fn blindsight_get(s: &mut Senses) -> bool {
            s.extra.blind_sight
        }

        #[rhai_fn(set = "blindsight")]
        pub fn blindsight_set(s: &mut Senses, v: bool) {
            s.extra.blind_sight = v;
        }

        #[rhai_fn(get = "truesight", pure)]
        pub fn truesight_get(s: &mut Senses) -> bool {
            s.extra.true_sight
        }

        #[rhai_fn(set = "truesight")]
        pub fn truesight_set(s: &mut Senses, v: bool) {
            s.extra.true_sight = v;
        }
    }

    // -- saving_throws submodule --

    pub mod saving_throws {
        use super::*;

        #[rhai_fn(get = "str", pure)]
        pub fn str_get(st: &mut SavingThrows) -> bool {
            st.str
        }

        #[rhai_fn(set = "str")]
        pub fn str_set(st: &mut SavingThrows, v: bool) {
            st.str = v;
        }

        #[rhai_fn(get = "dex", pure)]
        pub fn dex_get(st: &mut SavingThrows) -> bool {
            st.dex
        }

        #[rhai_fn(set = "dex")]
        pub fn dex_set(st: &mut SavingThrows, v: bool) {
            st.dex = v;
        }

        #[rhai_fn(get = "con", pure)]
        pub fn con_get(st: &mut SavingThrows) -> bool {
            st.con
        }

        #[rhai_fn(set = "con")]
        pub fn con_set(st: &mut SavingThrows, v: bool) {
            st.con = v;
        }

        #[rhai_fn(get = "int", pure)]
        pub fn int_get(st: &mut SavingThrows) -> bool {
            st.int
        }

        #[rhai_fn(set = "int")]
        pub fn int_set(st: &mut SavingThrows, v: bool) {
            st.int = v;
        }

        #[rhai_fn(get = "wis", pure)]
        pub fn wis_get(st: &mut SavingThrows) -> bool {
            st.wis
        }

        #[rhai_fn(set = "wis")]
        pub fn wis_set(st: &mut SavingThrows, v: bool) {
            st.wis = v;
        }

        #[rhai_fn(get = "cha", pure)]
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
        use std::str::FromStr;

        use ataxia_types::{Score, skills::Skill};
        use rhai::ImmutableString;

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

        pub fn set(
            overrides: &mut ataxia_types::skills::Overrides,
            skill: ImmutableString,
            score: Score,
        ) {
            let Ok(skill) = Skill::try_from(skill.as_str()) else {
                return;
            };
            match skill {
                Skill::Acrobatics => overrides.acrobatics = Some(score),
                Skill::AnimalHandling => overrides.animal_handling = Some(score),
                Skill::Arcana => overrides.arcana = Some(score),
                Skill::Athletics => overrides.athletics = Some(score),
                Skill::Deception => overrides.deception = Some(score),
                Skill::History => overrides.history = Some(score),
                Skill::Insight => overrides.insight = Some(score),
                Skill::Intimidation => overrides.intimidation = Some(score),
                Skill::Investigation => overrides.investigation = Some(score),
                Skill::Medicine => overrides.medicine = Some(score),
                Skill::Nature => overrides.nature = Some(score),
                Skill::Perception => overrides.perception = Some(score),
                Skill::Performance => overrides.performance = Some(score),
                Skill::Persuasion => overrides.persuasion = Some(score),
                Skill::Religion => overrides.religion = Some(score),
                Skill::SleightOfHand => overrides.sleight_of_hand = Some(score),
                Skill::Stealth => overrides.stealth = Some(score),
                Skill::Survival => overrides.survival = Some(score),
            }
        }
    }

    // -- scores submodule --
    pub mod scores {
        use ataxia_types::ability_score::AbilityScore;

        #[rhai_fn(pure)]
        pub fn get(score: &mut AbilityScore) -> i64 {
            score.get()
        }

        #[rhai_fn(name = "mod", name = "modifier", pure)]
        pub fn modifier(score: &mut AbilityScore) -> i64 {
            score.modifier()
        }

        #[rhai_fn(name = "mod", name = "modifier")]
        pub fn modifier_raw(s: i64) -> i64 {
            AbilityScore::new(s, 0).modifier()
        }
    }

    pub mod lore {

        use std::str::FromStr;

        use ataxia_types::lore::{Alignment, Morality, Order};
        use rhai::ImmutableString;

        use super::*;

        #[rhai_fn(set = "morality")]
        pub fn set_morality(a: &mut Alignment, v: ImmutableString) {
            let Ok(v) = Morality::from_str(v.as_str()) else {
                return;
            };
            a.morality = v;
        }

        #[rhai_fn(set = "order")]
        pub fn set_order(a: &mut Alignment, v: ImmutableString) {
            let Ok(v) = Order::from_str(v.as_str()) else {
                return;
            };
            a.order = v;
        }

        pub fn set(a: &mut Alignment, order: ImmutableString, morality: ImmutableString) {
            let Ok(m) = Morality::from_str(morality.as_str()) else {
                return;
            };
            let Ok(o) = Order::from_str(order.as_str()) else {
                return;
            };
            a.morality = m;
            a.order = o;
        }
    }

    pub mod language {
        use ataxia_types::language::{Language, Languages};
        use rhai::ImmutableString;

        pub fn add(l: &mut Languages, s: ImmutableString) {
            let s = Language::from(s.as_str());
            l.languages.push(s);
        }
    }

    pub mod feature {
        use ataxia_types::{feature::AbilityScoreBonus, senses::Sense, spells::Spell};
        use rhai::ImmutableString;

        #[rhai_fn(name = "feature")]
        pub fn feature_score(f: &mut Dynamic, score: Score, val: i64) {
            let a = f.take();
            match cast_into_feature(a) {
                Ok(mut a) => {
                    let effect = Effect::AbilityScoreBonus(AbilityScoreBonus {
                        score: score,
                        bonus: val as i32,
                    });
                    a.add(effect);

                    let a = match a {
                        FeatureEnum::Item(item) => Dynamic::from(item),
                        FeatureEnum::Feat(feat) => Dynamic::from(feat),
                    };
                    *f = a;
                }
                Err(a) => *f = a,
            }
        }

        #[rhai_fn(name = "feature")]
        pub fn feature_double(f: &mut Dynamic, i: ImmutableString, val: Dynamic) {
            let a = f.take();
            match cast_into_feature(a) {
                Ok(mut a) => {
                    let effect = parse_double_effect(i, val);
                    if let Some(effect) = effect {
                        a.add(effect);
                    } else {
                        return;
                    }

                    let a = match a {
                        FeatureEnum::Item(item) => Dynamic::from(item),
                        FeatureEnum::Feat(feat) => Dynamic::from(feat),
                    };
                    *f = a;
                }
                Err(a) => *f = a,
            }
        }

        #[rhai_fn(name = "feature")]
        pub fn feature_spell(f: &mut Dynamic, spell: Spell) {
            let a = f.take();
            match cast_into_feature(a) {
                Ok(mut a) => {
                    let effect = Effect::Spell(spell);
                    a.add(effect);

                    let a = match a {
                        FeatureEnum::Item(item) => Dynamic::from(item),
                        FeatureEnum::Feat(feat) => Dynamic::from(feat),
                    };
                    *f = a;
                }
                Err(a) => *f = a,
            }
        }

        #[rhai_fn(name = "feature")]
        pub fn feature_meter(f: &mut Dynamic, meter: Meter) {
            let a = f.take();
            match cast_into_feature(a) {
                Ok(mut a) => {
                    let effect = Effect::Meter(meter);
                    a.add(effect);

                    let a = match a {
                        FeatureEnum::Item(item) => Dynamic::from(item),
                        FeatureEnum::Feat(feat) => Dynamic::from(feat),
                    };
                    *f = a;
                }
                Err(a) => *f = a,
            }
        }

        #[rhai_fn(name = "feature")]
        pub fn feature_single(f: &mut Dynamic, i: ImmutableString) {
            let a = f.take();
            match cast_into_feature(a) {
                Ok(mut a) => {
                    let effect = {
                        if let Ok(s) = Skill::from_str(i.as_str()) {
                            Effect::AddProficiency(s)
                        } else {
                            return;
                        }
                    };
                    a.add(effect);

                    let a = match a {
                        FeatureEnum::Item(item) => Dynamic::from(item),
                        FeatureEnum::Feat(feat) => Dynamic::from(feat),
                    };
                    *f = a;
                }
                Err(a) => *f = a,
            }
        }
    }
}

/// Cast into a feature trait object. Returns the original value on failure.
fn cast_into_feature(f: Dynamic) -> Result<FeatureEnum, Dynamic> {
    match f.type_id() {
        id if id == TypeId::of::<Feat>() => Ok(FeatureEnum::Feat(f.cast())),
        id if id == TypeId::of::<Item>() => Ok(FeatureEnum::Item(f.cast())),
        _ => Err(f),
    }
}

enum FeatureEnum {
    Item(Item),
    Feat(Feat),
}

impl HasFeature for FeatureEnum {
    fn apply(&self, sheet: &mut ataxia_types::Character) {
        match self {
            FeatureEnum::Item(item) => item.apply(sheet),
            FeatureEnum::Feat(feat) => feat.apply(sheet),
        }
    }

    fn add(&mut self, feature: ataxia_types::feature::Effect) {
        match self {
            FeatureEnum::Item(item) => item.add(feature),
            FeatureEnum::Feat(feat) => feat.add(feature),
        }
    }
}

impl HasSource for FeatureEnum {
    fn source(&self) -> Option<&ataxia_types::source::Source> {
        match self {
            FeatureEnum::Item(item) => item.source(),
            FeatureEnum::Feat(feat) => feat.source(),
        }
    }

    fn add_source(&mut self, source: ataxia_types::source::Source) {
        match self {
            FeatureEnum::Item(item) => item.add_source(source),
            FeatureEnum::Feat(feat) => feat.add_source(source),
        }
    }
}

fn parse_double_effect(f: ImmutableString, val: Dynamic) -> Option<Effect> {
    let effect = match f.as_str() {
        "armor" | "armor_class" | "ac" => Some(Effect::AcBonus(val.as_int().ok()?)),
        "health_bonus" => Some(Effect::HealthBonusPerLevel(val.as_int().ok()?)),
        "initiative" | "initiative_bonus" => Some(Effect::InitiativeBonus(val.as_int().ok()?)),
        "expertise" => {
            if let Ok(skill) = Skill::from_str(&val.to_string()) {
                Some(Effect::Expertise(skill))
            } else {
                None
            }
        }
        "proficiency" => {
            if let Ok(skill) = Skill::from_str(&val.to_string()) {
                Some(Effect::AddProficiency(skill))
            } else {
                None
            }
        }
        _ => {
            let s = Sense::from_str(f.as_str()).ok();
            if let Some(s) = s {
                Some(Effect::Sense(s))
            } else {
                None
            }
        }
    };

    effect
}
